use super::admin::Admin;
use super::auth::Auth;
use super::database::Database;
use super::firestore::Firestore;
use super::credentials::Credentials;
use super::storage::Storage;
use super::messaging::Messaging;
use super::project_management::ProjectManagement;
use super::instance_id::InstanceId;
use std::collections::HashMap;
use futures::prelude::*;
use futures::future::Map;


/// A Firebase app holds the initialization information for a collection of services.
///
/// Do not call this constructor directly. Instead, use admin.initialize_app() to create an app.
#[derive(Clone)]
pub struct App {
    /// The (read-only) name for this app. The default app's name is "[DEFAULT]".
    pub name: String,
    pub options: AppOptions,
}

impl App {

    pub fn auth(&self) -> Auth {
        unimplemented!()
    }

    pub fn database(&self) -> Database {
        unimplemented!()
    }

    pub fn delete(&self) {
        unimplemented!();
    }

    pub fn firestore(&self) -> Firestore {
        Firestore {
            url: self.options.database_url.to_owned()
        }
    }

    pub fn instance_id(&self) -> InstanceId {
        unimplemented!()
    }

    pub fn messaging(&self) -> Messaging {
        unimplemented!()
    }

    pub fn project_management(&self) -> ProjectManagement {
        unimplemented!()
    }

    pub fn storage(&self) -> Storage {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct AppOptions {
    pub credentials: Credentials,
    pub database_auth_variable_override: HashMap<String, String>,
    pub database_url: String,
    pub http_agent: HashMap<String, String>,
    pub project_id: String,
    pub service_account_id: String,
    pub storage_bucket: String
}
