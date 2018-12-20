use std::{error::Error, fmt};
use std::collections::HashMap;

use super::app;
use super::credentials;


#[derive(Clone)]
pub struct Admin<'a> {
    pub apps: Vec<app::App<'a>>,
}

// TODO: Initialize SDK with google-service.json
impl<'a> Admin<'a> {
    /// Creates and initializes a Firebase app instance.
    /// 
    /// See Initialize the SDK and Initialize multiple apps for detailed documentation.
    pub fn app_builder() -> AppBuilder<'a> {
        
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

pub struct AppBuilder<'a> {
    name: Option<&'a str>,
    credentials: Option<credentials::Credentials>,
    database_auth_variable_override: Option<HashMap<&'a str, &'a str>>,
    database_url: Option<&'a str>,
    http_agent: Option<HashMap<&'a str, &'a str>>,
    project_id: Option<&'a str>,
    service_account_id: Option<&'a str>,
    storage_bucket: Option<&'a str>
}

impl<'a> AppBuilder<'a> {

    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_credentials(mut self, credentials_file: &'a str) -> Self {
        let credentials = credentials::Credentials::from_file(credentials_file);
        println!("{}", credentials.project_id);
        self.credentials = Some(credentials);
        self
    }

    pub fn with_database_url(mut self, database_url: &'a str) -> Self {
        self.database_url = Some(database_url);
        self
    }

    pub fn with_project_id(mut self, project_id: &'a str) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn with_http_agent(mut self, http_agent: HashMap<&'a str, &'a str>) -> Self {
        self.http_agent = Some(http_agent);
        self
    }

    pub fn with_service_account_id(mut self, service_account_id: &'a str) -> Self {
        self.service_account_id = Some(service_account_id);
        self
    }

    pub fn with_storage_bucket(mut self, storage_bucket: &'a str) -> Self {
        self.storage_bucket = Some(storage_bucket);
        self
    }

    /// Completes the build and initializes the app
    pub fn build_and_initialize(self) -> Result<app::App<'a>, String> {
        let credentials = self.credentials.unwrap().clone();
        let default_project_id = &credentials.project_id.clone();
        let project_id = self.project_id.unwrap_or(default_project_id);
        println!("{}", credentials.project_id);

        let built_app = app::App {
            name: self.name.unwrap_or("[DEFAULT]"),
            options: app::AppOptions {
                credentials: credentials.clone(),
                database_url: String::from(self.database_url.unwrap_or(&format!("{}.firebaseio.com", &project_id.clone()))),
                database_auth_variable_override: self.database_auth_variable_override.unwrap_or(HashMap::new()),
                http_agent: self.http_agent.unwrap_or(HashMap::new()),
                project_id: String::from(project_id),
                service_account_id: String::from(self.service_account_id.unwrap_or(&credentials.clone().client_id)),
                storage_bucket: String::from(self.storage_bucket.unwrap_or(&format!("{}.appspot.com", &project_id.clone())))
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
