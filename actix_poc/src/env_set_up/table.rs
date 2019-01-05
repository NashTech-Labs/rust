use env_set_up::connection::*;
use cdrs::query::QueryExecutor;
use constants::queries::TABLE;

pub fn create_table(session: &CurrentSession) {
    session.query(TABLE).expect("Table creation error");
}