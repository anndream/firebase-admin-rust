use std::{error::Error, fmt};
use std::collections::HashMap;

use super::app;
use super::credentials;


#[derive(Clone)]
pub struct Admin {
    pub apps: Vec<app::App>,
}

// TODO: Initialize SDK with google-service.json
impl Admin {
    /// Constructs an `AppBuilder` struct
    pub fn app_builder() -> AppBuilder<String> {
        
        AppBuilder {
            name: None,
            credentials: None,
            database_auth_variable_override: None,
            database_url: None,
            http_agent: None,
            project_id: None,
            service_account_id: None,
            storage_bucket: None
        }
        
    }

}

/// Allows you to build your options for your app before it is 
/// initialized.
pub struct AppBuilder<String> {
    name: Option<String>,
    credentials: Option<credentials::Credentials>,
    database_auth_variable_override: Option<HashMap<String, String>>,
    database_url: Option<String>,
    http_agent: Option<HashMap<String, String>>,
    project_id: Option<String>,
    service_account_id: Option<String>,
    storage_bucket: Option<String>
}

impl AppBuilder<String> {

    // Optionally set the name of your app. Defaults to `"[DEFAULT]"`.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Optionally specify the location of your app credentials JSON file.
    /// Defaults to looking for Application Default Credentials and panics if
    /// they cannot be found.
    pub fn with_credentials(mut self, credentials_file: &str) -> Self {
        let credentials = credentials::Credentials::from_file(credentials_file);
        println!("{}", credentials.project_id);
        self.credentials = Some(credentials);
        self
    }

    /// Optionally set the Database URL for your Firebase project.
    /// Defaults to `[project-id].firebaseio.com`
    pub fn with_database_url(mut self, database_url: &str) -> Self {
        self.database_url = Some(database_url.to_string());
        self
    }

    /// Optionally set the Project ID for the Firebase project you are
    /// using. Defaults to the Project ID specified in your credentials
    /// file.
    pub fn with_project_id(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_string());
        self
    }

    pub fn with_http_agent(mut self, http_agent: HashMap<String, String>) -> Self {
        self.http_agent = Some(http_agent);
        self
    }

    pub fn with_service_account_id(mut self, service_account_id: &str) -> Self {
        self.service_account_id = Some(service_account_id.to_string());
        self
    }

    /// Optionally set the Storage Bucket URL for your Firebase project.
    /// Defaults to `[project-id].appspot.com`.
    pub fn with_storage_bucket(mut self, storage_bucket: &str) -> Self {
        self.storage_bucket = Some(storage_bucket.to_string());
        self
    }

    /// Completes the build and initializes the app
    pub fn build_and_initialize(self) -> Result<app::App, String> {
        let credentials = self.credentials.unwrap().clone();
        let default_project_id = &credentials.project_id.clone();
        let project_id = self.project_id.unwrap_or(default_project_id.to_string());
        println!("{}", credentials.project_id);

        let built_app = app::App {
            name: self.name.unwrap_or("[DEFAULT]".to_string()),
            options: app::AppOptions {
                credentials: credentials.clone(),
                database_url: self.database_url.unwrap_or(format!("{}.firebaseio.com", &project_id.clone())),
                database_auth_variable_override: self.database_auth_variable_override.unwrap_or(HashMap::new()),
                http_agent: self.http_agent.unwrap_or(HashMap::new()),
                project_id: project_id.to_string(),
                service_account_id: String::from(self.service_account_id.unwrap_or(credentials.client_id)),
                storage_bucket: String::from(self.storage_bucket.unwrap_or(format!("{}.appspot.com", project_id)))
            }
        };

        Ok(built_app)
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
