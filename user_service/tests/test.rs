#[macro_use]
extern crate serde_json;
extern crate eventsourcing;
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
use user::user_service_impl::env_setup::initializer;
use actix_web::State;
use actix_web::Json;
use std::fmt::Error;
use user::user_service_impl::eventsourcing::user_state::UserState;
use user::user_service_impl::eventsourcing::user_entity::initial_state;
use user::user_service_impl::eventsourcing::user_command::UserCommand;
use user::user_service_impl::eventsourcing::user_event::UserEvent;
use user::user_service_impl::eventsourcing::user_entity::PUser;
use user::user_service_impl::eventsourcing::user_repository::event_persistent;
use user::user_service_impl::handler::map_user;
use user::error::CustomError;
use user::db_connection::CurrentSession;
use user::user_service_impl::handler::get_id_by_email;
use eventsourcing::*;
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

pub fn testing_down() {
    let user_delete_query = "DELETE user_event from user_event_sourcing_ks.user_events WHERE user_id = '42766930-3139-5385-b320-7bf6a6c199f1";

}

pub fn testing_setup() {
    initializer(&connect());
    static INDEX: usize = 0;
    let first_test_user = UserRegistration {
        name: "rudar".to_string(),
        email: "rudar@gmail.com".to_string(),
        password: "rudar@123".to_string(),
    };
    let second_test_user = UserRegistration {
        name: "sachin".to_string(),
        email: "sachin@gmail.com".to_string(),
        password: "sachinr@123".to_string(),
    };

    let session:CurrentSession = connect();
    let new_user_id: String = get_id_by_email(first_test_user.email.as_str()).to_string();
    let initial_user_state: UserState = initial_state();
    let create_user_command: UserCommand = UserCommand::CreateUser(first_test_user);
    let user_events: Vec<UserEvent> =
        PUser::handle_command(&initial_user_state, create_user_command).unwrap();
    let user_state: UserState =
        PUser::apply_event(&initial_user_state, user_events[INDEX].clone()).unwrap();
      let status = event_persistent(&session, &user_events[INDEX], new_user_id, &user_state);

    let new_user_id: String = get_id_by_email(second_test_user.email.as_str()).to_string();
    let initial_user_state: UserState = initial_state();
    let create_user_command: UserCommand = UserCommand::CreateUser(second_test_user);
    let user_events: Vec<UserEvent> =
        PUser::handle_command(&initial_user_state, create_user_command).unwrap();
    let user_state: UserState =
        PUser::apply_event(&initial_user_state, user_events[INDEX].clone()).unwrap();
    let status = event_persistent(&session, &user_events[INDEX], new_user_id, &user_state);

}



#[test]
fn test_insert_first_time() {
    testing_setup();
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
    testing_down();
}

#[test]
fn test_insert_not_first_time() {
    let user_reg: UserRegistration = UserRegistration {
        name: "sachin".to_string(),
        email: "sachin@gmail.com".to_string(),
        password: "sachinr@123".to_string(),
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
            "/get_user/26f2aedb-90cb-5e62-b321-52e9fc7f15a1",
        )
        .finish()
        .unwrap();

    let response: ClientResponse = server.execute(request.send()).unwrap();
    let user_detail = server.execute(response.body()).unwrap();
    let parsed_user_detail = str::from_utf8(&user_detail).unwrap();
    let user_detail_json: Value = serde_json::from_str(parsed_user_detail).unwrap();
    assert_eq!(
        user_detail_json,
        json!({"id":"26f2aedb-90cb-5e62-b321-52e9fc7f15a1","name":"rudar","email":"rudar@gmail.com"})
    );
}

#[test]
fn test_user_login() {
    let user_login: UserLogin = UserLogin {
        email: "sachin@gmail.com".to_string(),
        password: "sachinr@123".to_string(),
    };
    let mut server: TestServer = test::TestServer::with_factory(create_app);

    let request: ClientRequest = server
        .client(http::Method::POST, "/login")
        .json(user_login)
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();

    let user_id = server.execute(response.body()).unwrap();
    let parsed_user_id = str::from_utf8(&user_id).unwrap();
    assert_eq!(parsed_user_id, "7748941f-7a0c-5424-8687-16a14e6cbf0e");
}

#[test]
fn test_display_by_wrong_id() {
    let mut server: TestServer = test::TestServer::with_factory(create_app);
    let request: ClientRequest = server
        .client(
            http::Method::GET,
            "/get_user/1",
        )
        .finish()
        .unwrap();
    let response: ClientResponse = server.execute(request.send()).unwrap();
    assert!(response.status().is_client_error());
}

#[test]
fn test_user_login_not_exist() {
    let user_login: UserLogin = UserLogin {
        email: "amithaaaa@gmail.com".to_string(),
        password: "amithaaa@123".to_string(),
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



