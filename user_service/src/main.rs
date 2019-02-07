use actix_web::{http, server, App};
use listenfd::ListenFd;
use user::user_service_api::handler::AppState;
use user::user_service_impl::env_setup::initializer;
use user::user_service_impl::handler::UserInfo;
use user::user_service_api::handler::UserService;
use config::Config;
use user::db_connection::connect;

static INDEX: usize = 0;


#[cfg_attr(tarpaulin, skip)]
fn main() {
    let mut settings: Config = Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    let key: String = settings.get_str("debug_level_key").unwrap();
    let value: String = settings.get_str("debug_level_value").unwrap();
    let bind_port: String = settings.get_str("server_bind_port").unwrap();
    ::std::env::set_var(key, value);
    env_logger::init();
    initializer(&connect());

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState { session: connect() })
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
        server.bind(bind_port).unwrap()
    };

    server.run();
}
