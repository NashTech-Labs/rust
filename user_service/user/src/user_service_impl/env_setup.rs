use cdrs::query::QueryExecutor;
use crate::db_connection::CurrentSession;

static KEYSPACE_QUERY: &str =
    "CREATE KEYSPACE IF NOT EXISTS user_event_sourcing_ks WITH REPLICATION = {\
     'class' : 'SimpleStrategy', 'replication_factor' : 1 };";

static EVENT_TABLE_QUERY: &str =
    "CREATE TABLE IF NOT EXISTS user_event_sourcing_ks.user_events\
     (user_id text PRIMARY KEY , user_event text);";

static STATE_TABLE_QUERY: &str =
    "CREATE TABLE IF NOT EXISTS user_event_sourcing_ks.user_states \
     (user_id text PRIMARY KEY ,user_state text);";

/// create_keyspace takes Current Session and keyspace_name
/// * and creates a keyspace in database and return string
fn create_keyspace(session: &CurrentSession) -> &'static str {
    session
        .query(KEYSPACE_QUERY)
        .expect("keyspace creation error");
    "keyspace created successfully"
}

/// create_table takes Current Session and table_name
/// * and create tables in database and return string
fn create_table(session: &CurrentSession) -> &'static str {
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
