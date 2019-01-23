extern crate actix_web;
extern crate scylladb_poc;

use actix_web::client::ClientRequest;
use actix_web::test::TestServer;
use actix_web::{client::ClientResponse, HttpMessage};
use actix_web::{http, test, App};
use scylladb_poc::controller::handler::AppState;
use scylladb_poc::controller::handler::{delete, insert, show, update};
use scylladb_poc::env_set_up::connection::connect;
use scylladb_poc::env_set_up::set_up::initializer;
use scylladb_poc::models::model::Student;
use std::str;

#[cfg_attr(tarpaulin, skip)]
fn create_app() -> App<AppState> {
    initializer(&connect());
    App::with_state(AppState { session: connect() })
        .resource("/add", |r| r.with(insert))
        .resource("/show/{roll_no}", |r| r.with(show))
        .resource("/delete/{roll_no}", |r| r.with(delete))
        .resource("/update/{roll_no}", |r| r.with(update))
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_insert_first_time() {
    let student = Student {
        roll_no: 12,
        name: "amita".to_string(),
        marks: 65,
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::POST, "/add")
        .json(student)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "welcome! student added");
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_insert_not_first_time() {
    let student = Student {
        roll_no: 1,
        name: "ayush".to_string(),
        marks: 80,
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::POST, "/add")
        .json(student)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student already exist");
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_display() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::GET, "/show/1")
        .finish()
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "{\"roll_no\":1,\"name\":\"ayush\",\"marks\":80}");
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_update() {
    let student = Student {
        roll_no: 1,
        name: "ayush".to_string(),
        marks: 80,
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::PUT, "/update/1")
        .json(student)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student with this id updated");
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_update_student_not_exist() {
    let student = Student {
        roll_no: 47,
        name: "ayush".to_string(),
        marks: 80,
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::PUT, "/update/47")
        .json(student)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student doesn't exist");
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_display_student_not_exist() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::GET, "/show/19")
        .finish()
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "{\"roll_no\":0,\"name\":\"\",\"marks\":0}");
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_delete() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::DELETE, "/delete/45")
        .finish()
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student deleted..");
}

#[cfg_attr(tarpaulin, skip)]
#[test]
fn test_delete_student_not_exist() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::DELETE, "/delete/2")
        .finish()
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student doesn't exist..");
}

#[test]
fn test_student_name_empty()
{
    let student = Student {
        roll_no: 12,
        name: "".to_string(),
        marks: 65,
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::POST, "/add")
        .json(student)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "");
}



#[test]
fn test_student_marks_less_than_max_marks()
{
    let student = Student {
        roll_no: 12,
        name: "amita".to_string(),
        marks: 165,
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::POST, "/add")
        .json(student)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    let bytes = server.execute(response.body()).unwrap();
    let body: &str = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "");
}