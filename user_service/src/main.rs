#[macro_use]
extern crate lazy_static;
extern crate time;

use actix_web::{http, server, App};
use listenfd::ListenFd;
use user::user_service_api::user_service::AppState;
use user::user_service_impl::env_setup::initializer;
use user::user_service_impl::handler::UserInfo;
use user::user_service_api::user_service::UserService;
use config::Config;
use user::db_connection::connect;
use std::error::Error;
use std::sync::RwLock;
use actix_web::middleware::session::{CookieSessionBackend, SessionStorage};
use time::Duration;
static INDEX: usize = 0;

lazy_static! {
	static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

struct ConfigSetting {
    debug_level_key: String,
    debug_level_value: String,
    server_bind_port: String,
}

impl ConfigSetting {
    fn new() -> Result<ConfigSetting, Box<Error>> {
        /// Set property
        SETTINGS.write()?.set("debug_level_key", "RUST_LOG")?;
        SETTINGS.write()?.set("debug_level_value", "actix_web=debug")?;
        SETTINGS.write()?.set("server_bind_port", "127.0.0.1:3080")?;

        /// Get property
        let key: String = SETTINGS.read()?.get_str("debug_level_key")?;
        let value: String = SETTINGS.read()?.get_str("debug_level_value")?;
        let port: String = SETTINGS.read()?.get_str("server_bind_port")?;

        Ok(ConfigSetting {
            debug_level_key: key,
            debug_level_value: value,
            server_bind_port: port
        })
    }
}
#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_setting: ConfigSetting = ConfigSetting::new()?;
    ::std::env::set_var(config_setting.debug_level_key, config_setting.debug_level_value);
    env_logger::init();
    initializer(&connect());

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState { session: connect() })
            .middleware(SessionStorage::new(
                CookieSessionBackend::signed(&[0;32])
                    .max_age(Duration::minutes(2))
                    .secure(false)
            ))
            .resource("/create_user", |r| {
                r.method(http::Method::POST).with_async(UserInfo::create_user)
            })
            .resource("/login", |r| {
                r.method(http::Method::POST).with_async(UserInfo::user_login)
            })
            .resource("/get_user/{user_id}", |r| {
                r.method(http::Method::GET).with_async(UserInfo::get_user)
            })
            .resource("/get_users", |r| {
                r.method(http::Method::GET).with_async(UserInfo::get_all_users)
            })
    });
    server = if let Some(listen) = listenfd.take_tcp_listener(INDEX).unwrap() {
        server.listen(listen)
    } else {
        server.bind(config_setting.server_bind_port).unwrap()
    };

    server.run();
    Ok(())
}
