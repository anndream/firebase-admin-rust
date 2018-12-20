/*!
 # Firebase Admin SDK for Rust
 */
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate futures;
extern crate config;
#[macro_use]
extern crate serde_derive;

pub mod admin;
pub mod app;
pub mod auth;
pub mod credentials;
pub mod database;
pub mod firestore;
mod instance_id;
pub mod messaging;
pub mod project_management;
pub mod storage;


