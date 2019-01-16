use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::load_balancing::RoundRobin;
use crate::constants::constants::DATABASE_PORT_ADDRESS;

///creating a custom type of Current Session type
pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

/// connect function is used to create CurrentSession for database operation
pub fn connect() -> CurrentSession {
    let node = NodeTcpConfigBuilder::new(DATABASE_PORT_ADDRESS, NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession = new_session(&cluster_config, RoundRobin::new())
        .expect("session should be created");
    no_compression
}