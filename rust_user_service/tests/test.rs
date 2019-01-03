extern crate actix_web;

use actix_web::{App, http,test};
use actix_web::{HttpMessage,client::ClientResponse};
use std::str;
use user_service::user_service_impl::controller::handler::AppState;
use user_service::user_service_impl::env_setup::connection::connect;
use user_service::user_service_impl::controller::handler::initializer;
use user_service::user_service_impl::controller::handler::create_user;
use user_service::user_service_impl::controller::handler::user_login;
use user_service::user_service_impl::controller::handler::get_user;
use user_service::user_service_impl::controller::handler::get_all_users;
use user_service::user_service_impl::models::user_registration::UserRegistration;
use actix_web::test::TestServer;
use actix_web::client::ClientRequest;
use serde_json::Value;

fn create_app() -> App<AppState> {
    App::with_state(AppState { session: connect() })
        .resource("/set_up", |r| r.method(http::Method::GET).with(initializer))
        .resource("/create_user", |r| {
            r.method(http::Method::POST).with(create_user)
        })
           /* .resource("/login", |r| r.method(http::Method::POST).with(user_login))
            .resource("/get_user/{user_id}", |r| {
                r.method(http::Method::GET).with(get_user)
            })
            .resource("/get_user", |r| {
                r.method(http::Method::GET).with(get_all_users)
            })*/
}

#[test]
fn test_initializer() {
    let mut srv: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = srv.client(http::Method::GET, "/set_up").finish()
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();

    assert_eq!(body, "successfully up");
}

#[test]
fn test_insert_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "amita".to_string(),
        email: "mai@gmail.com".to_string(),
        password: "amita".to_string()
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = srv.client(http::Method::POST, "/create_user").json(user_reg)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let struct_body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(struct_body, "{'email': 'mai@gmail.com', 'id': '2f6f4c06-753c-5d29-9cec-324d7168577c', 'name' : 'amita'}");
}


#[test]
fn test_insert_not_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "rahul".to_string(),
        email: "rsb007@gmail.com".to_string(),
        password: "rsb007@".to_string()
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = srv.client(http::Method::POST, "/create_user").json(user_reg)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
}

/*
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
*/
//}