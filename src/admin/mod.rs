extern crate reqwest;

use std::{error::Error, fmt};
use std::collections::HashMap;


pub mod app;
pub mod auth;
pub mod credential;
pub mod database;
pub mod firestore;
pub mod instance_id;
pub mod messaging;
pub mod project_management;
pub mod storage;

#[derive(Clone)]
pub struct Admin {
    apps: Vec<app::App>,
}

// TODO: Initialize SDK with google-service.json
impl Admin {

}

#[derive(Debug)]
pub struct FirebaseError {
    code: String,
    message: String,
    stack: String
}

impl FirebaseError {
    pub fn to_json() {

    }
}

impl Error for FirebaseError {}

impl fmt::Display for FirebaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub struct FirebaseArrayIndexError {
    error: FirebaseError,
    index: i32
}



