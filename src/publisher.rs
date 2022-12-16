use crate::{book::Book, HTTP_CLIENT};
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = " Recently read book \u{01f4da}";

#[derive(Debug, PartialEq, Eq)]
pub enum PublishError {
    GistError(String),
}

#[derive(Serialize, Deserialize)]
struct PatchGistRequest {
    files: RequestFiles,
}
impl PatchGistRequest {
    pub fn new(book: Book) -> Self {
        PatchGistRequest {
            files: RequestFiles {
                recently_read_book: RecentlyReadBook {
                    filename: FILE_NAME.to_string(),
                    content: book.content(),
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequestFiles {
    recently_read_book: RecentlyReadBook,
}

#[derive(Serialize, Deserialize)]
struct RecentlyReadBook {
    filename: String,
    content: String,
}

pub trait Publisher {
    fn publish(&self, book: Book) -> Result<(), PublishError>;
}

pub struct Gist {
    pub url: String,
    pub gh_token: String,
}
impl Gist {
    pub fn new(url: String, gh_token: String) -> Self {
        Gist { url, gh_token }
    }
}

impl Publisher for Gist {
    fn publish(&self, book: Book) -> Result<(), PublishError> {
        HTTP_CLIENT
            .patch(&self.url)
            .header("Authorization", &self.gh_token)
            .json(&PatchGistRequest::new(book))
            .send()
            .map_err(|e| PublishError::GistError(e.to_string()))
            .and_then(|response| match response.status().is_success() {
                true => Ok(()),
                false => Err(PublishError::GistError(response.text().unwrap())),
            })
    }
}
