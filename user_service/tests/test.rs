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
use user::user_service_api::user_service::AppState;
use user::db_connection::connect;
use user::user_service_impl::handler::UserInfo;
use user::user_service_api::user_service::UserService;
use cdrs::query::QueryExecutor;
use user::user_service_impl::env_setup::initializer;

#[cfg_attr(tarpaulin, skip)]
fn create_app() -> App<AppState> {
    initializer(&connect());
    App::with_state(AppState { session: connect() })
        .resource("/create_user", |r| {
            r.method(http::Method::POST).with_async(UserInfo::create_user)})
        .resource("/login", |r|{ r.method(http::Method::POST)
            .with_async(UserInfo::user_login)})
        .resource("/get_user/{user_id}", |r| {
            r.method(http::Method::GET).with_async(UserInfo::get_user)
        })
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

    connect().query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'a9c8536e-75ee-582b-a145-b6ace45abe9d'")
        .expect("Deletion error in insert handler test");
}

#[test]
fn test_insert_not_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "sid".to_string(),
        email: "sid@gmail.com".to_string(),
        password: "sid123@".to_string(),
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    {
        let request: ClientRequest = server
            .client(http::Method::POST, "/create_user")
            .json(user_reg.clone())
            .unwrap();
        let _response: ClientResponse = server.execute(request.send()).unwrap();
    }
    let request: ClientRequest = server
        .client(http::Method::POST, "/create_user")
        .json(user_reg)
        .unwrap();

    let response: ClientResponse = server.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
    connect().query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'a9c8536e-75ee-582b-a145-b6ace45abe9d'")
        .expect("Deletion error in insert handler test");
}

#[test]
fn test_display_by_id() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let user_reg: UserRegistration = UserRegistration {
        name: "sid".to_string(),
        email: "sid@gmail.com".to_string(),
        password: "sid123@".to_string(),
    };
    let request: ClientRequest = server
        .client(http::Method::POST, "/create_user")
        .json(user_reg)
        .unwrap();
    let _response: ClientResponse = server.execute(request.send()).unwrap();
    let request: ClientRequest = server
        .client(
            http::Method::GET,
            "/get_user/a9c8536e-75ee-582b-a145-b6ace45abe9d",
        )
        .finish()
        .unwrap();

    let response: ClientResponse = server.execute(request.send()).unwrap();
    let user_detail = server.execute(response.body()).unwrap();
    let parsed_user_detail = str::from_utf8(&user_detail).unwrap();
    let user_detail_json: Value = serde_json::from_str(parsed_user_detail).unwrap();
    assert_eq!(
        user_detail_json,
        json!({"id": "a9c8536e-75ee-582b-a145-b6ace45abe9d","name": "sid","email":
    "sid@gmail.com"})
    );
    connect().query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'a9c8536e-75ee-582b-a145-b6ace45abe9d'")
        .expect("Deletion error in insert handler test");
}

#[test]
fn test_user_login() {
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
    let _response: ClientResponse = server.execute(request.send()).unwrap();
    let user_login: UserLogin = UserLogin {
        email: "sid@gmail.com".to_string(),
        password: "sid123@".to_string(),
    };
    let request: ClientRequest = server
        .client(http::Method::POST, "/login")
        .json(user_login)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    let user_id = server.execute(response.body()).unwrap();
    let parsed_user_id = str::from_utf8(&user_id).unwrap();
    assert_eq!(parsed_user_id, "a9c8536e-75ee-582b-a145-b6ace45abe9d");
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
