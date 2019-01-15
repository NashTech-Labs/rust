use crate::env_setup::connection::CurrentSession;
use crate::env_setup::keyspace::create_keyspace;
use crate::env_setup::table::create_table;

/// initializer is used to create keyspace and tables
/// takes state which provide session for queries' execution
pub fn initializer(session: &CurrentSession) -> &str {
    create_keyspace(&session);
    create_table(&session);
    "environment successfully up"
}