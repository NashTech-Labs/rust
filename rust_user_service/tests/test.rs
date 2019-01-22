extern crate actix_web;
#[macro_use]
extern crate serde_json;
use actix_web::{App, http, test};
use actix_web::{HttpMessage, client::ClientResponse};
use std::str;
use user_service::user_service_impl::controller::handler::AppState;
use user_service::user_service_impl::env_setup::connection::connect;
use user_service::user_service_impl::controller::handler::create_user;
use user_service::user_service_impl::controller::handler::user_login;
use user_service::user_service_impl::controller::handler::get_user;
use user_service::user_service_impl::controller::handler::get_all_users;
use user_service::user_service_impl::models::user_registration::UserRegistration;
use actix_web::test::TestServer;
use actix_web::client::ClientRequest;
use serde_json::Value;
use user_service::user_service_impl::models::user_login::UserLogin;
use user_service::user_service_impl::env_setup::set_up::initializer;

fn create_app() -> App<AppState> {
    initializer(&connect());

    App::with_state(AppState { session: connect() })
        .resource("/create_user", |r| {
            r.method(http::Method::POST).with(create_user)
        })
        .resource("/login", |r| r.method(http::Method::POST).with(user_login))
            .resource("/get_user/{user_id}", |r| {
                r.method(http::Method::GET).with(get_user)
            })
        .resource("/login", |r| r.method(http::Method::POST)
            .with(user_login))
        .resource("/get_user", |r| {
            r.method(http::Method::GET).f(get_all_users)
        })
}

#[test]
fn test_initializer() {
   assert_eq!(initializer(&connect()), "environment successfully up");
}

#[test]
fn test_insert_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "shikha".to_string(),
        email: "shikha97887@gmail.com".to_string(),
        password: "shikha123".to_string(),
     };
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server.client(http::Method::POST,
                                            "/create_user").json(user_reg)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    assert!(response.status().is_success());
   /* let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let user_detail: Value = serde_json::from_str(body).unwrap();
    assert_eq!(user_detail, "{'email': 'shikha@gmail.com', 'id':
     '8eea6a91-2c44-5dfd-b889-39992ab8d510', 'name' : 'shikha'}");
   */
}

#[test]
fn test_insert_not_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "rahul".to_string(),
        email: "rsb007@gmail.com".to_string(),
        password: "rsb007@".to_string(),
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = srv.client(http::Method::POST,
                                            "/create_user").json(user_reg)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    assert!(response.status().is_client_error());
}

#[test]
fn test_display_by_id() {
    let mut server = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server.
        client(http::Method::GET, "/get_user/e4ba964b-43e9-50a2-9eae-cb2039000ccd")
        .finish().unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    let user_in_bytes = server.execute(response.body()).unwrap();
    let user_in_string = str::from_utf8(&user_in_bytes).unwrap();
    let user_detail: Value = serde_json::from_str(user_in_string).unwrap();
    let expect_user_detail = json!({"email": "shikha@gmail.com", "id": "e4ba964b-43e9-50a2-9eae-cb2039000ccd", "name": "shikha"});
    assert_eq!(user_detail, expect_user_detail);
}

#[test]
fn test_user_login() {
    let user: UserLogin = UserLogin {
        email: "rsb007@gmail.com".to_string(),
        password: "rsb007@".to_string()
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server.client(http::Method::POST, "/login").json(user)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    let user_in_bytes = server.execute(response.body()).unwrap();
    let user_id_in_string = str::from_utf8(&user_in_bytes).unwrap();
    assert_eq!(user_id_in_string, "f95dfd0b-e2fa-5b88-a284-578f9a015f4d");
}

#[test]
fn test_display_by_wrong_id() {
    let mut server = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server.client(http::Method::GET, "/get_user/9216d4b7-3f05-5118-88d4-2daa9ec67418").finish().unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    let user_in_bytes = server.execute(response.body()).unwrap();
    let user_in_string = str::from_utf8(&user_in_bytes).unwrap();
    let user_detail: Value = serde_json::from_str(user_in_string).unwrap();
    assert_eq!(user_detail, json!({"id": "9216d4b7-3f05-5118-88d4-2daa9ec67418","name": "abhishek","email": "abhishek@gmail.com"}));
}

#[test]
fn test_user_login_not_exist() {
    let user: UserLogin = UserLogin {
        email: "rahul@gmail.com".to_string(),
        password: "rsb007@".to_string()
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server.client(http::Method::POST, "/login/").json(user)
        .unwrap();
    let response:ClientResponse = server.execute(request.send()).unwrap();

    let user_in_bytes = server.execute(response.body()).unwrap();
    let user_in_string = str::from_utf8(&user_in_bytes).unwrap();
    assert_eq!(user_in_string, "");
}

#[test]
fn test_display_all_users() {
    let mut server = test::TestServer::with_factory(create_app);
    let request = match server.client(http::Method::GET, "/get_users").finish().unwrap();

    let response: ClientResponse = server.execute(request.send()).unwrap();
   // assert!(response.status().is_success());
    let bytes = server.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let user_detail: Value = serde_json::from_str(body).unwrap();
    assert_eq!(user_detail, "{
       'outcomes': [
           {
               'id': 'c6fd1799-b363-57f5-a4f5-6bfc12cef619',
               'name': 'shikha',
               'email: 'shikha97887@gmail.com'
           },
           {
               'id': '3275d519-28e5-5707-94a6-d16fac19835f',
               'name': 'rohit',
               'email': 'rahul@gmail.com'
           },
           {
               'id': 'dbf95ae8-6ee5-57fe-9d48-be8f6475cc8f',
               'name': 'amita',
               'email': 'amita@gmail.com'
           },
           {
               'id': 'f95dfd0b-e2fa-5b88-a284-578f9a015f4d',
               'name': 'rahul',
               'email': 'rsb007@gmail.com'
           }
       ]
   }");
}
