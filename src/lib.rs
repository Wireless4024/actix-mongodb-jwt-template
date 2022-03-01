//! Actix Mongodb Jwt template
//! this template is personal template used in private project

#![warn(missing_docs)]
#![forbid(unused_imports)]
#![forbid(unsafe_code)]

/// this module contains utilities and helpers
pub mod util;

/// this module contains stuff to make authorization
pub mod auth;

/// this module contains struct to store data in database
pub mod schema;

/// this module use to provide routing
pub mod routes;

/// this module having controller that help to manage schema
pub mod controller;

/// this module contains helper in web handle
pub mod web;