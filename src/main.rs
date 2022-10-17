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

#[derive(Debug, Bson, Mongo)]
#[mongo(collection = "users", field, filter, update)]
struct User {
    username: String,
    password: String,
    registration_time: f64,
}

fn main() {}
