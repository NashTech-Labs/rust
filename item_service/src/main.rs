extern crate actix_web;
extern crate env_logger;
extern crate listenfd;

use actix_web::{http, server, App};
use listenfd::ListenFd;

use actix_web::middleware;
use item::item_service_impl::constants::constants::DEBUG_LEVEL_KEY;
use item::item_service_impl::constants::constants::DEBUG_LEVEL_VALUE;
use item::item_service_impl::env_setup::set_up::initializer;
use item::item_service_impl::env_setup::connection::connect;
use item::item_service_api::item_service::AppState;
use item::item_service_api::item_service::start_auction;
use item::item_service_api::item_service::{get_item,create_item, get_items_for_user, start_auction,
                                           update_item};
use item::item_service_impl::constants::constants::ZERO;
use item::item_service_impl::constants::constants::SERVER_BIND_PORT;

fn main() {
    ::std::env::set_var(DEBUG_LEVEL_KEY, DEBUG_LEVEL_VALUE);
    env_logger::init();
    initializer(&connect());

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState { session: connect() })
            .middleware(middleware::DefaultHeaders::new().header("",""))
            .resource("/create_item", |r| {
                r.method(http::Method::POST).with(create_item);
                //r.method(http::Method::HEAD).with(create_item);
            })
            .resource("/start_auction", |r| r.method(http::Method::POST)
                .with(start_auction))
            .resource("/get_item/{item_id}", |r| {
                r.method(http::Method::GET).with(get_item)
            })
            .resource("/update_item/{item_id}", |r| {
                r.method(http::Method::PUT).with(update_item)
            }).resource("/get_user_items", |r| {
                r.method(http::Method::GET).f(get_items_for_user)
            })
    });
    server = if let Some(l) = listenfd.take_tcp_listener(ZERO).unwrap() {
        server.listen(l)
    } else {
        server.bind(SERVER_BIND_PORT).unwrap()
    };

    server.run();
}