extern crate scylladb_poc;
extern crate actix_web;

use actix_web::{App, http,test};
use actix_web::{HttpMessage,client::ClientResponse};
use scylladb_poc::{models::model::Student,handlers::handler::{insert,delete,show,update}};
use std::str;

fn create_app() -> App {
    App::new()
        .resource("/add", |r| r.with(insert))
        .resource("/show/{roll_no}", |r| r.with(show))
        .resource("/delete/{roll_no}", |r| r
            .with(delete))
        .resource("/update/{roll_no}", |r| r
            .with(update))
}

#[test]
fn test_insert_first_time() {
    let stu = Student { roll_no: 11, name: "amita".to_string(), marks: 65 };
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::POST, "/add").json(stu)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "welcome! student added");
}

#[test]
fn test_insert_not_first_time() {
    let stu = Student { roll_no: 1, name: "ayush".to_string(), marks: 80 };
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::POST, "/add").json(stu)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student already exist");
}


#[test]
fn test_display() {
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::GET, "/show/1").finish().unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "{\"roll_no\":1,\"name\":\"ayush\",\"marks\":80}");
}

#[test]
fn test_update() {
    let stu = Student { roll_no: 1, name: "ayush".to_string(), marks: 80 };
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::PUT, "/update/1").json(stu)
        .unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student with this id updated");
}

#[test]
fn test_update_student_not_exist() {
    let stu = Student { roll_no: 1, name: "ayush".to_string(), marks: 80 };
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::PUT, "/update/11").json(stu)
        .unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student doesn't exist");
}



#[test]
fn test_display_student_not_exist() {
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::GET, "/show/12").finish()
        .unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "{\"roll_no\":0,\"name\":\"\",\"marks\":0}");

}

#[test]
fn test_delete() {
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::DELETE, "/delete/1").finish()
        .unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student deleted..");
}

#[test]
fn test_delete_student_not_exist() {
    let mut srv = test::TestServer::with_factory(create_app);

    let request = srv.client(http::Method::DELETE, "/delete/1").finish()
        .unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "student doesn't exist..");
}