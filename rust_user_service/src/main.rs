extern crate actix_web;
extern crate env_logger;
extern crate listenfd;
extern crate user_service;

use actix_web::{http, server, App};
use listenfd::ListenFd;

use eventsourcing::eventstore::MemoryEventStore;
use user_service::user_service_impl::constants::constant::SERVER_BIND_PORT;
use user_service::user_service_impl::env_setup::set_up::initializer;
use user_service::user_service_impl::controller::handler::{
    create_user, get_all_users, get_user, user_login, AppState,
};
use user_service::user_service_impl::env_setup::connection::connect;
use user_service::user_service_impl::constants::constant::TAKE_FIRST;
use user_service::user_service_impl::constants::constant::DEBUG_LEVEL_KEY;
use user_service::user_service_impl::constants::constant::DEBUG_LEVEL_VALUE;

fn main() {
    let _user_store: MemoryEventStore = MemoryEventStore::new();
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
    server = if let Some(l) = listenfd.take_tcp_listener(TAKE_FIRST).unwrap() {
        server.listen(l)
    } else {
        server.bind(SERVER_BIND_PORT).unwrap()
    };

    server.run();
}
