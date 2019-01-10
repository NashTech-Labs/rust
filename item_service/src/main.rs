extern crate actix_web;
extern crate env_logger;
extern crate listenfd;
extern crate item_service_impl;

use actix_web::{http, server, App};
use listenfd::ListenFd;

use item_service_impl::constants::constants::DEBUG_LEVEL_KEY;
use item_service_impl::constants::constants::DEBUG_LEVEL_VALUE;
use item_service_impl::constants::constants::SERVER_BIND_PORT;
use item_service_impl::constants::constants::TAKE_FIRST;
use item_service_impl::controller::handler::create_item;
use item_service_impl::controller::handler::start_auction;
use item_service_impl::controller::handler::get_item;
use item_service_impl::controller::handler::update_item;
use item_service_impl::controller::handler::get_items_for_user;

fn main() {
    ::std::env::set_var(DEBUG_LEVEL_KEY, DEBUG_LEVEL_VALUE);
    env_logger::init();
    initializer(&connect());

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState { session: connect() })
            .resource("/create_item", |r| {
                r.method(http::Method::POST).with(create_item)
            })
            .resource("/start_auction", |r| r.method(http::Method::POST)
                .with(start_auction))
            .resource("/get_item", |r| {
                r.method(http::Method::GET).with(get_item)
            })
            .resource("/update_item", |r| {
                r.method(http::Method::PUT).with(update_item)
            }).resource("/get_user_items", |r| {
                r.method(http::Method::GET).f(get_items_for_user)
            })
    });
    server = if let Some(l) = listenfd.take_tcp_listener(TAKE_FIRST).unwrap() {
        server.listen(l)
    } else {
        server.bind(SERVER_BIND_PORT).unwrap()
    };

    server.run();
}