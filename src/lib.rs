mod book;
mod publisher;

use book::{BookManager, Booklog, WorkflowError};
use once_cell::sync::{Lazy, OnceCell};
use publisher::{Gist, PublishError, Publisher};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug)]
pub enum ApplicationError {
    FetchError(WorkflowError),
    PublishError(PublishError),
}

impl From<WorkflowError> for ApplicationError {
    fn from(e: WorkflowError) -> Self {
        ApplicationError::FetchError(e)
    }
}

const USER_AGENT: &str = "Mozilla/5.0";

pub static CONFIG: OnceCell<Config> = OnceCell::new();
pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Failed to load config")
}

#[derive(Debug, Deserialize)]
pub struct Config {
    gh_token: String,
    gist_id: String,
    booklog_user_id: String,
}

impl Config {
    pub fn init() {
        let config = envy::from_env::<Config>().expect("Failed to load config");
        CONFIG.set(config).expect("Failed to set config");
    }
}

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to build HTTP client")
});

pub fn run() -> Result<(), ApplicationError> {
    let url = format!("https://api.github.com/gists/{}", get_config().gist_id);
    let token = format!("token {}", get_config().gh_token);

    let book = Booklog::new()
        .fetch_recently_read_book()
        .map_err(ApplicationError::FetchError)?;

    Gist::new(url, token)
        .publish(book)
        .map_err(ApplicationError::PublishError)
}
