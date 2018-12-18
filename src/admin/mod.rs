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
    /// Creates and initializes a Firebase app instance.
    /// 
    /// See Initialize the SDK and Initialize multiple apps for detailed documentation.
    pub fn initialize_app(options: Option<app::AppOptions>, name: Option<String>) -> Result<app::App, String> {
        
        let default_app_options: app::AppOptions = app::AppOptions {
            credential: Some(credential::Credential {}),
            database_auth_variable_override: HashMap::new(),
            database_url: String::from(""),
            http_agent: HashMap::new(),
            project_id: String::from(""),
            service_account_id: String::from(""),
            storage_bucket: String::from("")
        };

        let app = app::App {
            name: name.unwrap_or(String::from("[DEFAULT]")),
            options: options.unwrap_or(default_app_options)
        };
        
        Ok(app)
    }
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
