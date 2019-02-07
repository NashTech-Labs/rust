use cdrs::authenticators::NoneAuthenticator;
/*use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;*/
use cdrs::query::QueryExecutor;
use crate::db_connection::CurrentSession;

pub static KEYSPACE_QUERY: &str =
    "CREATE KEYSPACE IF NOT EXISTS user_event_sourcing_ks WITH REPLICATION = {\
     'class' : 'SimpleStrategy', 'replication_factor' : 1 };";

pub static EVENT_TABLE_QUERY: &str =
    "CREATE TABLE IF NOT EXISTS user_event_sourcing_ks.user_events\
     (user_id text PRIMARY KEY , user_event text);";

pub static STATE_TABLE_QUERY: &str =
    "CREATE TABLE IF NOT EXISTS user_event_sourcing_ks.user_states \
     (user_id text PRIMARY KEY ,user_state text);";

/*pub static DATABASE_PORT_ADDRESS: &str = "127.0.0.1:9042";

///creating a custom type of Current Session type
pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

/// connect function is used to create CurrentSession for database operation
#[cfg_attr(tarpaulin, skip)]
pub fn connect() -> CurrentSession {
    let node: NodeTcpConfig<NoneAuthenticator> =
        NodeTcpConfigBuilder::new(DATABASE_PORT_ADDRESS, NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");
    no_compression
}*/

/// create_keyspace takes Current Session and keyspace_name
/// * and creates a keyspace in database and return string
pub fn create_keyspace(session: &CurrentSession) -> &'static str {
    session
        .query(KEYSPACE_QUERY)
        .expect("keyspace creation error");
    "keyspace created successfully"
}

/// create_table takes Current Session and table_name
/// * and create tables in database and return string
pub fn create_table(session: &CurrentSession) -> &'static str {
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

#[test]
fn test_create_table() {
    assert_eq!("Tables created successfully", create_table(&connect()))
}

#[test]
fn test_keyspace() {
    assert_eq!("keyspace created successfully", create_keyspace(&connect()))
}
