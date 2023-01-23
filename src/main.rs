use std::process;

use recently_read_book::ApplicationError;

extern crate recently_read_book;

fn main() {
    recently_read_book::Config::init();
    match recently_read_book::run() {
        Ok(_) => process::exit(0),
        Err(ApplicationError::FetchError(e)) => panic!("Failed to fetch: {:?}", e),
        Err(ApplicationError::PublishError(e)) => panic!("Failed to publish: {:?}", e),
    }
}
