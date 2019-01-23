use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::NodeTcpConfig;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use constants::constant::DATABASE_PORT_ADDRESS;

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

pub fn connect() -> CurrentSession {
    let node: NodeTcpConfig<NoneAuthenticator> =
        NodeTcpConfigBuilder::new(DATABASE_PORT_ADDRESS, NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");
    no_compression
}
