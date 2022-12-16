use crate::{get_config, HTTP_CLIENT};
use scraper::{Html, Selector};

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    title: String,
    author: String,
}

impl Book {
    pub fn new(title: String, author: String) -> Self {
        Book { title, author }
    }

    pub fn content(&self) -> String {
        format!("{:?}\n{}", self.title, self.author)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkflowError {
    ScrapeError(String),
    ParseError,
    ExtractError,
}

impl From<reqwest::Error> for WorkflowError {
    fn from(e: reqwest::Error) -> Self {
        WorkflowError::ScrapeError(e.to_string())
    }
}

pub trait BookManager {
    fn fetch_recently_read_book(&self) -> Result<Book, WorkflowError>;
}

pub struct Booklog {}
impl Booklog {
    pub fn new() -> Self {
        Booklog {}
    }
}

impl BookManager for Booklog {
    fn fetch_recently_read_book(&self) -> Result<Book, WorkflowError> {
        can_scrape().and_then(scrape).and_then(parse)
    }
}

const BOOKLOG_HOST: &str = "https://booklog.jp";

fn can_scrape() -> Result<bool, WorkflowError> {
    Ok(HTTP_CLIENT
        .get(format!("{}/{}", BOOKLOG_HOST, "robots.txt"))
        .send()?
        .text()?
        .contains("Disallow: /users/*/all"))
}

fn scrape(_: bool) -> Result<String, WorkflowError> {
    HTTP_CLIENT
        .get(format!(
            "{}/users/{}/all?status=3&sort=read_desc",
            BOOKLOG_HOST,
            get_config().booklog_user_id
        ))
        .send()?
        .text()
        .map_err(|e| WorkflowError::ScrapeError(e.to_string()))
}

fn parse(html_string: String) -> Result<Book, WorkflowError> {
    Selector::parse(".item-area-img")
        .map_err(|_| WorkflowError::ParseError)
        .and_then(|selector| {
            Html::parse_document(&html_string)
                .select(&selector)
                .next()
                .and_then(|e| e.value().attr("title"))
                .ok_or(WorkflowError::ParseError)
                .and_then(extract_book_attribute)
        })
}

fn extract_book_attribute(html_string: &str) -> Result<Book, WorkflowError> {
    let extract = |s: &str| s.trim().to_string();

    let author = html_string.split('『').next().map(extract);
    let title = html_string
        .split('『')
        .nth(1)
        .and_then(|x| x.split('』').next().map(extract));

    match (author, title) {
        (None, _) => Err(WorkflowError::ExtractError),
        (_, None) => Err(WorkflowError::ExtractError),
        (Some(author), Some(title)) => Ok(Book::new(title, author)),
    }
}

#[cfg(test)]
mod tests {
    use crate::book::{extract_book_attribute, Book};

    #[test]
    fn test_parse_from_html_string_to_book() {
        let book = extract_book_attribute("author『title』");
        let expect = Ok(Book::new("title".to_string(), "author".to_string()));

        assert_eq!(book, expect);
    }
}
