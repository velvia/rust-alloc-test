use serde_json::error::{Error as SJError};
use json::{Error as JsonError};

#[derive(Debug)]
pub enum MyError {
    IOError(std::io::Error),
    SerdeJsonError(SJError),
    JsonError(JsonError),
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

impl From<JsonError> for MyError {
    fn from(err: JsonError) -> MyError {
        MyError::JsonError(err)
    }
}