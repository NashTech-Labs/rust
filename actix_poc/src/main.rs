extern crate actix_web;
extern crate env_logger;
extern crate listenfd;
extern crate scylladb_poc;
extern crate serde_json;

use scylladb_poc::env_set_up::connection::connect;

use scylladb_poc::controller::handler::{delete, insert, show, update};

use actix_web::{http, server, App};
use listenfd::ListenFd;
use scylladb_poc::constants::constant::SERVER_BIND_PORT;
use scylladb_poc::constants::constant::DEBUG_LEVEL_VALUE;
use scylladb_poc::constants::constant::DEBUG_LEVEL_KEY;
use scylladb_poc::env_set_up::set_up::initializer;
use scylladb_poc::controller::handler::AppState;
use scylladb_poc::constants::constant::ZERO;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    ::std::env::set_var(DEBUG_LEVEL_KEY, DEBUG_LEVEL_VALUE);
    env_logger::init();

    initializer(&connect());
    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::with_state(AppState { session: connect() })
            .resource("/add", |r| r.method(http::Method::POST).with(insert))
            .resource("/show/{roll_no}", |r| {
                r.method(http::Method::GET).with(show)
            })
            .resource("/delete/{roll_no}", |r| {
                r.method(http::Method::DELETE).with(delete)
            })
            .resource("/update/{roll_no}", |r| {
                r.method(http::Method::PUT).with(update)
            })
    });
    server = if let Some(listen) = listenfd.take_tcp_listener(ZERO).unwrap() {
        server.listen(listen)
    } else {
        server.bind(SERVER_BIND_PORT).unwrap()
    };

    server.run();
}
