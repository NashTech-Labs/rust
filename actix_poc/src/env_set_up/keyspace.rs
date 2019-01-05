use env_set_up::connection::*;
use cdrs::query::QueryExecutor;
use constants::queries::KEYSPACE;

pub fn create_keyspace(session: &CurrentSession) {
   session.query(KEYSPACE).expect("keyspace creation error");
}