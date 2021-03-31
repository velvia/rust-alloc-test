use serde_json::error::{Error as SJError};

#[derive(Debug)]
pub enum MyError {
    IOError(std::io::Error),
    SerdeJsonError(SJError)
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::IOError(err)
    }
}

impl From<SJError> for MyError {
    fn from(err: SJError) -> MyError {
        MyError::SerdeJsonError(err)
    }
}