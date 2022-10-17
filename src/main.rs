use mongod::{Bson, Mongo};

use thiserror::Error;

type Result<T> = core::result::Result<T, MyError>;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("")]
    EvenWithThisImpl(#[from] mongod::Error),
    #[error("unknown data store error")]
    Unknown,
}

// Replace the dependency in toml from path to the 0.3.1 release to reproduce the error

#[derive(Debug, Bson, Mongo)]
#[mongo(collection = "users", field, filter, update)]
struct User {
    username: String,
    password: String,
    registration_time: f64,
}

fn main() {}
