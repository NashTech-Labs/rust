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
/*
use user_service::user_service_impl::controller::handler::get_all_users;
*/
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
            .resource("/login", |r| r.method(http::Method::POST).with(user_login))
            .resource("/get_user/{user_id}", |r| {
                r.method(http::Method::GET).with(get_user)
            })/*
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

    assert_eq!(body, "environment successfully up");
}

#[test]
fn test_insert_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "shikha".to_string(),
        email: "shikha97887@gmail.com".to_string(),
        password: "shikha123".to_string()
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = srv.client(http::Method::POST, "/create_user").json(user_reg)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

   /* let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let struct_body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(struct_body, "{'email': 'shikha@gmail.com', 'id': '8eea6a91-2c44-5dfd-b889-39992ab8d510', 'name' : 'shikha'}");*/
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


#[test]
fn test_display() {
    let mut srv = test::TestServer::with_factory(create_app);

    let request: ClientRequest = srv.client(http::Method::GET, "/get_user/9216d4b7-3f05-5118-88d4-2daa9ec67418").finish().unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();

    let struct_body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(struct_body, "{'id': '9216d4b7-3f05-5118-88d4-2daa9ec67418','name': 'abhishek','email': 'abhishek@gmail.com'}");
}

#[test]
fn test_user_login() {
    let user_reg: UserRegistration = UserRegistration {
        name: "rahul".to_string(),
        email: "rsb007@gmail.com".to_string(),
        password: "rsb007@".to_string()
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = srv.client(http::Method::POST, "/login").json(user_reg)
        .unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "f95dfd0b-e2fa-5b88-a284-578f9a015f4d");
}
#[test]
fn test_user_login_not_exist() {
    let user_reg: UserRegistration = UserRegistration {
        name: "rahul".to_string(),
        email: "rahul@gmail.com".to_string(),
        password: "rsb007@".to_string()
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = srv.client(http::Method::POST, "/login").json(user_reg)
        .unwrap();
    let response:ClientResponse = srv.execute(request.send()).unwrap();

    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "");
}


/*
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