use actix_web::{http, server, App};
use listenfd::ListenFd;
use user::user_service_impl::env_setup::initializer;
use user::user_service_impl::env_setup::connect;
use user::user_service_api::handler::AppState;
use user::user_service_impl::handler::{create_user, get_user, get_all_users, user_login};


pub static DEBUG_LEVEL_KEY: &str = "RUST_LOG";

pub static DEBUG_LEVEL_VALUE: &str = "actix_web=debug";

pub static SERVER_BIND_PORT: &str = "127.0.0.1:3080";

pub static INDEX: usize = 0;

#[cfg_attr(tarpaulin,skip)]
fn main() {
    ::std::env::set_var(DEBUG_LEVEL_KEY, DEBUG_LEVEL_VALUE);
    env_logger::init();
    initializer(&connect());

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState { session: connect() })
            .resource("/create_user", |r| {
                r.method(http::Method::POST).with_async(create_user)
            })
            .resource("/login", |r| r.method(http::Method::POST)
                .with_async(user_login))
            .resource("/get_user/{user_id}", |r| {
                r.method(http::Method::GET).with_async(get_user)
            })
          .resource("/get_users", |r| {
              r.method(http::Method::GET).with_async(get_all_users)
              })
          });
    server = if let Some(listen) = listenfd.take_tcp_listener(INDEX).unwrap() {
        server.listen(listen)
    } else {
        server.bind(SERVER_BIND_PORT).unwrap()
    };

    server.run();
}
