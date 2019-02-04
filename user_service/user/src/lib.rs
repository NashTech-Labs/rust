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

pub mod user_service_api;
pub mod user_service_impl;