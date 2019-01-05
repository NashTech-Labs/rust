extern crate actix_web;
extern crate listenfd;
extern crate scylladb_poc;
extern crate serde_json;
extern crate env_logger;

use scylladb_poc::env_set_up::{connection::connect,keyspace::create_keyspace,
                               table::create_table};
use scylladb_poc::handlers::handler::{insert,delete,show,update};

use actix_web::{App,http, server};
use listenfd::ListenFd;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let session = connect();
    create_keyspace(&session);
    create_table(&session);

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/add", |r| r.method(http::Method::POST)
                .with(insert))
            .resource("/show/{roll_no}", |r| r.method(http::Method::GET)
                .with(show))
            .resource("/delete/{roll_no}", |r| r
                .method(http::Method::DELETE).with(delete))
            .resource("/update/{roll_no}", |r| r.method(http::Method::PUT)
                .with(update))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3080").unwrap()
    };

    server.run();
}