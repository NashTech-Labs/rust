extern crate user;

use actix_web::{http, server, App};
use listenfd::ListenFd;

use user::user_service_impl::constants::constant::DEBUG_LEVEL_KEY;
use user::user_service_impl::constants::constant::DEBUG_LEVEL_VALUE;
use user::user_service_impl::env_setup::set_up::initializer;
use user::user_service_impl::env_setup::connection::connect;
use user::user_service_api::user_service::handler::AppState;
use user::user_service_api::user_service::handler::create_user;
use user::user_service_api::user_service::handler::user_login;
use user::user_service_api::user_service::handler::get_user;
use user::user_service_api::user_service::handler::get_all_users;
use user::user_service_impl::constants::constant::INDEX;
use user::user_service_impl::constants::constant::SERVER_BIND_PORT;

#[cfg_attr(tarpaulin,skip)]
fn main() {
    ::std::env::set_var(DEBUG_LEVEL_KEY, DEBUG_LEVEL_VALUE);
    env_logger::init();
    initializer(&connect());

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState { session: connect() })
            .resource("/create_user", |r| {
                r.method(http::Method::POST).with(create_user)
            })
            .resource("/login", |r| r.method(http::Method::POST)
                .with(user_login))
            .resource("/get_user/{user_id}", |r| {
                r.method(http::Method::GET).with(get_user)
            })
          .resource("/get_users", |r| {
              r.method(http::Method::GET).f(get_all_users)
              })
          });
    server = if let Some(listen) = listenfd.take_tcp_listener(INDEX).unwrap() {
        server.listen(listen)
    } else {
        server.bind(SERVER_BIND_PORT).unwrap()
    };

    server.run();
}
