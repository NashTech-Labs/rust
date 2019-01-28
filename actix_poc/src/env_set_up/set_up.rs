use env_set_up::connection::CurrentSession;
use env_set_up::keyspace::create_keyspace;
use env_set_up::table::create_table;

/// initializer is used to create keyspace and tables
/// takes state which provide session for queries' execution
pub fn initializer(session: &CurrentSession) -> &str {
    create_keyspace(&session);
    create_table(&session);
    "environment successfully up"
}
