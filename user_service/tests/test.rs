#[macro_use]
extern crate serde_json;

use actix_web::client::ClientRequest;
use actix_web::test::TestServer;
use actix_web::{client::ClientResponse, HttpMessage};
use actix_web::{http, test, App};
use serde_json::Value;
use std::str;
use user::model::UserLogin;
use user::model::UserRegistration;
use user::user_service_api::handler::AppState;
use user::db_connection::connect;
use user::user_service_impl::handler::UserInfo;
use user::user_service_api::handler::UserService;

#[cfg_attr(tarpaulin, skip)]
fn create_app() -> App<AppState> {
    App::with_state(AppState { session: connect() })
        .resource("/create_user", |r| {
            r.method(http::Method::POST).with_async(UserInfo::create_user)
        })
        .resource("/login", |r| r.method(http::Method::POST).with_async(UserInfo::user_login))
        .resource("/get_user/{user_id}", |r| {
            r.method(http::Method::GET).with(UserInfo::get_user)
        })
        .resource("/login", |r| r.method(http::Method::POST).with_async(UserInfo::user_login))
        .resource("/get_users", |r| {
            r.method(http::Method::GET).with_async(UserInfo::get_all_users)
        })
}

#[test]
fn test_insert_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "sid".to_string(),
        email: "sid@gmail.com".to_string(),
        password: "sid123@".to_string(),
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server
        .client(http::Method::POST, "/create_user")
        .json(user_reg)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    assert!(response.status().is_success());
    let user_detail_in_bytes = server.execute(response.body()).unwrap();
    let parsed_user_detail = str::from_utf8(&user_detail_in_bytes).unwrap();
    let user_detail: Value = serde_json::from_str(parsed_user_detail).unwrap();
    assert_eq!(
        user_detail,
        json!({"email": "sid@gmail.com", "id":
     "a9c8536e-75ee-582b-a145-b6ace45abe9d", "name" : "sid"})
    );
}

#[test]
fn test_insert_not_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "rahul".to_string(),
        email: "rsb007@gmail.com".to_string(),
        password: "rsb007@".to_string(),
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server
        .client(http::Method::POST, "/create_user")
        .json(user_reg)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    assert!(response.status().is_client_error());
}

#[test]
fn test_display_by_id() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server
        .client(
            http::Method::GET,
            "/get_user/3275d519-28e5-5707-94a6-d16fac19835f",
        )
        .finish()
        .unwrap();

    let response: ClientResponse = server.execute(request.send()).unwrap();
    let user_detail = server.execute(response.body()).unwrap();
    let parsed_user_detail = str::from_utf8(&user_detail).unwrap();
    let user_detail_json: Value = serde_json::from_str(parsed_user_detail).unwrap();
    assert_eq!(
        user_detail_json,
        json!({"id": "3275d519-28e5-5707-94a6-d16fac19835f","name": "rohit","email":
    "rahul@gmail.com"})
    );
}

#[test]
fn test_user_login() {
    let user_login: UserLogin = UserLogin {
        email: "rsb007@gmail.com".to_string(),
        password: "rsb007@".to_string(),
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::POST, "/login")
        .json(user_login)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    let user_id = server.execute(response.body()).unwrap();
    let parsed_user_id = str::from_utf8(&user_id).unwrap();
    assert_eq!(parsed_user_id, "f95dfd0b-e2fa-5b88-a284-578f9a015f4d");
}

#[test]
fn test_display_by_wrong_id() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server
        .client(
            http::Method::GET,
            "/get_user/9216d4b7-3f05-5118-88d4-2daa9ec67418",
        )
        .finish()
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    assert!(response.status().is_client_error());
}

#[test]
fn test_user_login_not_exist() {
    let user_login: UserLogin = UserLogin {
        email: "rahul@gmail.com".to_string(),
        password: "rsb007@".to_string(),
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::POST, "/login")
        .json(user_login)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    assert!(response.status().is_client_error());
}

#[test]
fn test_display_all_users() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server
        .client(http::Method::GET, "/get_users")
        .finish()
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    assert!(response.status().is_success());
}
