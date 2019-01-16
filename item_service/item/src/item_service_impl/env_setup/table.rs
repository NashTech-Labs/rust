use cdrs::query::QueryExecutor;

use crate::constants::queries::{EVENT_TABLE_QUERY, STATE_TABLE_QUERY};
use crate::env_setup::connection::CurrentSession;

/// create_table takes Current Session and table_name
/// * and create tables in database and return string
pub fn create_table(session: &CurrentSession) -> &'static str {
    session.query(EVENT_TABLE_QUERY).expect("Event Table creation error");
    session.query(STATE_TABLE_QUERY).expect("State Table creation error");

    "Tables created successfully"
}

#[test]
fn test_create_table() {
    use crate::env_setup::connection::connect;
    assert_eq!("Tables created successfully",
               create_table(&connect()));
}
