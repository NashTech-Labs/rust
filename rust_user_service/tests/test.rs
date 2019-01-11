extern crate actix_web;

use actix_web::{App, http, test};
use actix_web::{HttpMessage, client::ClientResponse};
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
use user_service::user_service_impl::models::user_login::UserLogin;

fn create_app() -> App<AppState> {
    App::with_state(AppState { session: connect() })
        .resource("/set_up", |r| r.method(http::Method::GET)
            .with(initializer))
        .resource("/create_user", |r| {
            r.method(http::Method::POST).with(create_user)
        })
<<<<<<< HEAD
            .resource("/login", |r| r.method(http::Method::POST).with(user_login))
            .resource("/get_user/{user_id}", |r| {
                r.method(http::Method::GET).with(get_user)
            })/*
            .resource("/get_user", |r| {
                r.method(http::Method::GET).with(get_all_users)
            })*/
=======
        .resource("/login", |r| r.method(http::Method::POST)
            .with(user_login))
        .resource("/get_user/{user_id}", |r| {
            r.method(http::Method::GET).with(get_user)
        })
        .resource("/get_user", |r| {
            r.method(http::Method::GET).f(get_all_users)
        })
>>>>>>> 94334322fddb5eacaafb99cc2707c5f28874c647
}

#[test]
fn test_initializer() {
    let mut srv: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = srv.client(http::Method::GET, "/set_up")
        .finish()
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());
    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
<<<<<<< HEAD

=======
>>>>>>> 94334322fddb5eacaafb99cc2707c5f28874c647
    assert_eq!(body, "environment successfully up");
}

#[test]
fn test_insert_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "shikha".to_string(),
        email: "shikha97887@gmail.com".to_string(),
<<<<<<< HEAD
        password: "shikha123".to_string()
=======
        password: "shikha123".to_string(),
>>>>>>> 94334322fddb5eacaafb99cc2707c5f28874c647
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = srv.client(http::Method::POST,
                                            "/create_user").json(user_reg)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());
<<<<<<< HEAD

   /* let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let struct_body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(struct_body, "{'email': 'shikha@gmail.com', 'id': '8eea6a91-2c44-5dfd-b889-39992ab8d510', 'name' : 'shikha'}");*/
=======
    /* let bytes = srv.execute(response.body()).unwrap();
     let body = str::from_utf8(&bytes).unwrap();
     let struct_body: Value = serde_json::from_str(body).unwrap();
     assert_eq!(struct_body, "{'email': 'shikha@gmail.com', 'id': '8eea6a91-2c44-5dfd-b889-39992ab8d510', 'name' : 'shikha'}");*/
>>>>>>> 94334322fddb5eacaafb99cc2707c5f28874c647
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
    let mut srv = test::TestServer::with_factory(create_app);
    let request: ClientRequest = srv.
        client(http::Method::GET, "/get_user/3275d519-28e5-5707-94a6-d16fac19835f")
        .finish().unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let struct_body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(struct_body, "{'id': '3275d519-28e5-5707-94a6-d16fac19835f','name': 'rohit','email': 'rahul@gmail.com'}");
}

#[test]
<<<<<<< HEAD
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
=======
fn test_display_by_wrong_id() {
    let mut srv = test::TestServer::with_factory(create_app);
    let request: ClientRequest = srv.client(http::Method::GET, "/get_user/9216d4b7-3f05-5118-88d4-2daa9ec67418").finish().unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let struct_body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(struct_body, "{'id': '9216d4b7-3f05-5118-88d4-2daa9ec67418','name': 'abhishek','email': 'abhishek@gmail.com'}");
>>>>>>> 94334322fddb5eacaafb99cc2707c5f28874c647
}
#[test]
<<<<<<< HEAD
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
=======
fn test_user_login() {
    let user_login: UserLogin = UserLogin {
        email: "rahul@gmail.com".to_string(),
        password: "poiuytrewq".to_string(),
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = srv.client(http::Method::POST, "/login")
        .json(user_login)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "3275d519-28e5-5707-94a6-d16fac19835f");
}

>>>>>>> 94334322fddb5eacaafb99cc2707c5f28874c647
#[test]
fn test_user_login_not_exist() {
    let user_login: UserLogin = UserLogin {
        email: "rsb0017@gmail.com".to_string(),
        password: "rsb0017@".to_string(),
    };
    let mut srv: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = srv.client(http::Method::POST, "/login")
        .json(user_login)
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    assert_eq!(body, "");
}

#[test]
fn test_display_all_users() {
    let mut srv = test::TestServer::with_factory(create_app);
    let request = srv.client(http::Method::GET, "/get_users").finish()
        .unwrap();
    let response: ClientResponse = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());
    let bytes = srv.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    let struct_body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(struct_body, "{
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
