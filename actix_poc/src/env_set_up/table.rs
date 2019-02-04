use cdrs::query::QueryExecutor;
use constants::queries::TABLE;
use env_set_up::connection::*;

/// Create table using connection on scylla db
pub fn create_table(session: &CurrentSession) {
    session.query(TABLE).expect("Table creation error");
}
