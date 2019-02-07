use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use config::Config;
use cdrs::authenticators::NoneAuthenticator;

//pub static DATABASE_PORT_ADDRESS: &str = "127.0.0.1:9042";

///creating a custom type of Current Session type
pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

/// connect function is used to create CurrentSession for database operation
#[cfg_attr(tarpaulin, skip)]
pub fn connect() -> CurrentSession {
    /*let mut configuration: Config = Config::default();
    configuration.merge(config::File::with_name("Configuration")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    let port:&str =configuration.get("database_port_address").unwrap();*/
    let node: NodeTcpConfig<NoneAuthenticator> =
        NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");
    no_compression
}