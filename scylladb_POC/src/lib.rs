#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
//extern crate mock_derive;

pub mod env_set_up {
    pub mod connection;
    pub mod keyspace;
    pub mod table;
    pub mod models;
}

pub mod crud {
    pub mod insert;
    pub mod delete;
    pub mod update;
    pub mod display;
}

pub mod test;