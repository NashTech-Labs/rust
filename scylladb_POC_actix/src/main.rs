extern crate actix_web;
extern crate listenfd;
extern crate scylladb_poc;
extern crate serde_json;

mod handler;
use handler::*;

use actix_web::{App,http, server};
use listenfd::ListenFd;

fn main() {
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

#[cfg(test)]
mod test;