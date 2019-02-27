#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate eventsourcing;
#[macro_use]
extern crate eventsourcing_derive;
extern crate uuid;
extern crate config;
#[macro_use]
extern crate validator_derive;
extern crate validator;

pub mod user_service_api {
    pub mod user_service;
}

pub mod user_service_impl;

pub mod error;

pub mod model;

pub mod utility;

pub mod db_connection;

pub mod constants;
