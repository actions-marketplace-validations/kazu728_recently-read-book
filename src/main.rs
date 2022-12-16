use recently_read_book::ApplicationError;

extern crate recently_read_book;

fn main() {
    recently_read_book::Config::init();
    match recently_read_book::run() {
        Ok(_) => println!("Done"),
        Err(ApplicationError::FetchError(e)) => panic!("Failed to fetch: {:?}", e),
        Err(ApplicationError::PublishError(e)) => panic!("Failed to publish: {:?}", e),
    }
}
