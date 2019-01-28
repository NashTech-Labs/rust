use cdrs::query::QueryExecutor;
use constants::queries::KEYSPACE;
use env_set_up::connection::*;

/// Create keyspace using connection on scylla db
pub fn create_keyspace(session: &CurrentSession) {
    session.query(KEYSPACE).expect("keyspace creation error");
}
