use super::credential::Credential;
use std::collections::HashMap;

#[derive(Clone)]
pub struct App {
    /// The (read-only) name for this app
    /// 
    /// The default app's name is "[DEFAULT]".
    pub name: String,
    pub options: AppOptions
}

impl App {
    pub fn auth() {

    }

    pub fn database() {
        
    }

    pub fn delete() {
        
    }

    pub fn firestore() {
        
    }

    pub fn instance_id() {

    }

    pub fn messaging() {

    }

    pub fn project_management() {

    }

    pub fn storage() {

    }
}

#[derive(Clone)]
pub struct AppOptions {
    pub credential: Option<Credential>,
    pub database_auth_variable_override: HashMap<String, String>,
    pub database_url: String,
    pub http_agent: HashMap<String, String>,
    pub project_id: String,
    pub service_account_id: String,
    pub storage_bucket: String
}
