use cdrs::query::QueryExecutor;

use crate::constants::queries::KEYSPACE_QUERY;
use crate::env_setup::connection::CurrentSession;

/// create_keyspace takes Current Session and keyspace_name
/// * and creates a keyspace in database and return string
pub fn create_keyspace(session: &CurrentSession) -> &'static str {
    session.query(KEYSPACE_QUERY).expect("keyspace creation error");
    "keyspace created successfully"
}

#[test]
fn test_keyspace() {
    use crate::user_service_impl::env_setup::connection::connect;
    assert_eq!("keyspace created successfully",create_keyspace(&connect()))
}