use cdrs::query::QueryExecutor;
use crate::db_connection::CurrentSession;
use std::fs;
use glob::glob;
use config::*;
use std::collections::HashMap;
use crate::user_service_impl::eventsourcing::user_repository::configration_reader;

/// create_keyspace takes Current Session and keyspace_name
/// * and creates a keyspace in database and return string
fn create_keyspace(session: &CurrentSession) -> &'static str {
    //let KEYSPACE_QUERY =
    session
        .query(configration_reader().get("keyspace_query").unwrap())
        .expect("keyspace creation error");
    "keyspace created successfully"
}

/// create_table takes Current Session and table_name
/// * and create tables in database and return string
fn create_table(session: &CurrentSession) -> &'static str {

    let EVENT_TABLE_QUERY = configration_reader().get("event_table_query").unwrap().to_owned();
    let STATE_TABLE_QUERY = configration_reader().get("state_table_query").unwrap().to_owned();

    session
        .query(EVENT_TABLE_QUERY)
        .expect("Event Table creation error");
    session
        .query(STATE_TABLE_QUERY)
        .expect("State Table creation error");

    "Tables created successfully"
}

/// initializer is used to create keyspace and tables
/// takes state which provide session for queries' execution
pub fn initializer(session: &CurrentSession) -> &str {
    create_keyspace(&session);
    create_table(&session);
    "environment successfully up"
}

#[cfg(test)]
mod tests {
    use crate::user_service_impl::env_setup::create_table;
    use crate::user_service_impl::env_setup::create_keyspace;
    use crate::db_connection::connect;
    use crate::user_service_impl::env_setup::initializer;

    #[test]
    fn test_initializer() {
        assert_eq!(initializer(&connect()), "environment successfully up");
    }

    #[test]
    fn test_create_table() {
        assert_eq!("Tables created successfully", create_table(&connect()))
    }

    #[test]
    fn test_keyspace() {
        assert_eq!("keyspace created successfully", create_keyspace(&connect()))
    }
}
