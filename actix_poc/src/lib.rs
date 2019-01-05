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

pub mod env_set_up {
    pub mod connection;
    pub mod keyspace;
    pub mod table;
}

pub mod crud {
    pub mod insert;
    pub mod delete;
    pub mod update;
    pub mod display;
    pub mod is_present;
}

pub mod models {
    pub mod model;
}

pub mod error;

pub mod handlers{
    pub mod handler;
}

pub mod constants;