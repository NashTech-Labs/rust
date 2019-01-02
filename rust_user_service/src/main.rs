extern crate actix_web;
extern crate listenfd;
extern crate env_logger;
extern crate user_service;

use actix_web::{App,http,server};
use listenfd::ListenFd;
use std::thread;

use user_service::user_service_impl::controller::handler::{AppState,create_user/*,user_login,get_user
                                                           ,get_all_users*/};
use user_service::user_service_impl::constants::constant::SERVER_BIND_PORT;
use std::cell::Cell;
use user_service::user_service_impl::env_setup::connection::connect;
use user_service::user_service_impl::controller::handler::intializer;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState {session:connect()})
            .resource("/set_up", |r| r.method(http::Method::GET)
                .with(intializer))
            .resource("/create_user", |r| r.method(http::Method::POST)
                .with(create_user))
            /*.resource("/login", |r| r.method(http::Method::POST)
                .with(user_login))
            .resource("/get_user/{user_id}", |r| r.method(http::Method::GET)
                .with(get_user))
            .resource("/get_user", |r| r.method(http::Method::GET)
                .with(get_all_users))*/
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(SERVER_BIND_PORT).unwrap()
    };

    server.run();
}