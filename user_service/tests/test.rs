#[macro_use]
extern crate serde_json;

use actix_web::client::ClientRequest;
use actix_web::test::TestServer;
use actix_web::{client::ClientResponse, HttpMessage};
use actix_web::{http, test, App};
use serde_json::Value;
use std::str;
use user::model::User;
use user::model::UserLogin;
use user::model::UserRegistration;
use user::user_service_api::handler::AppState;
use user::user_service_impl::env_setup::connect;
use user::user_service_impl::env_setup::initializer;
use user::user_service_impl::eventsourcing::user_entity::initial_state;
use user::user_service_impl::eventsourcing::user_entity::PUser;
use user::user_service_impl::eventsourcing::user_event::UserEvent;
use user::user_service_impl::eventsourcing::user_repository::event_persistent;
use user::user_service_impl::eventsourcing::user_repository::is_present;
use user::user_service_impl::eventsourcing::user_repository::select_all_user;
use user::user_service_impl::eventsourcing::user_repository::select_user;
use user::user_service_impl::eventsourcing::user_repository::UserMapper;
use user::user_service_impl::eventsourcing::user_state::UserState;
use user::user_service_impl::handler::get_id_by_email;
use user::user_service_impl::handler::map_user;
use user::wrapper::wrap_vec;
use user::wrapper::Outcomes;

fn set_up_db() {
    let session = connect();
}
#[cfg_attr(tarpaulin, skip)]
fn create_app() -> App<AppState> {
    App::with_state(AppState { session: connect() })
        .resource("/create_user", |r| {
            r.method(http::Method::POST).with(create_user)
        })
        .resource("/login", |r| r.method(http::Method::POST).with(user_login))
        .resource("/get_user/{user_id}", |r| {
            r.method(http::Method::GET).with(get_user)
        })
        .resource("/login", |r| r.method(http::Method::POST).with(user_login))
        .resource("/get_users", |r| {
            r.method(http::Method::GET).f(get_all_users)
        })
}

#[test]
fn test_initializer() {
    assert_eq!(initializer(&connect()), "environment successfully up");
}

#[test]
fn test_get_id_by_email() {
    assert_eq!(
        get_id_by_email("sid@gmail.com").to_string(),
        "a9c8536e-75ee-582b-a145-b6ace45abe9d".to_string()
    )
}

#[test]
fn test_initial_state() {
    assert_eq!(
        initial_state(),
        UserState {
            user: PUser {
                id: "".to_string(),
                name: "".to_string(),
                email: "".to_string(),
                password: "".to_string(),
            },
            generation: 0,
        }
    )
}

#[test]
fn test_wrap_vec() {
    let user_list: Vec<User> = vec![
        User {
            id: "101".to_string(),
            name: "sanjay".to_string(),
            email: "sanjay@gmail.com".to_string(),
        },
        User {
            id: "102".to_string(),
            name: "sunil".to_string(),
            email: "sunil@gmail.com".to_string(),
        },
    ];
    let outcomes: Outcomes<User> = Outcomes {
        outcomes: user_list.clone(),
    };

    assert_eq!(wrap_vec(user_list), outcomes);
}

#[test]
fn test_map_user() {
    assert_eq!(
        map_user(PUser {
            id: String::new(),
            name: String::new(),
            email: String::new(),
            password: String::new()
        }),
        User {
            id: String::new(),
            name: String::new(),
            email: String::new()
        }
    )
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

#[test]
fn test_select_user() {
    let user_mapper: UserMapper = UserMapper {
        user_id: "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
        user_state:
            "{\"user\":{\"id\":\"c6fd1799-b363-57f5-a4f5-6bfc12cef619\",\"name\":\"shikha\",\
             \"email\":\"shikha97887@gmail.com\",\"password\":\"shikha123\"},\"generation\":1}"
                .to_string(),
    };
    let user_detail: Vec<UserMapper> = vec![user_mapper];
    assert_eq!(
        select_user(
            &connect(),
            "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string()
        ),
        user_detail
    )
}

#[test]
fn test_select_all_user() {
    assert_eq!(select_all_user(&connect()).len(), 8)
}

#[test]
fn test_select_user_not_exist() {
    assert!(select_user(
        &connect(),
        "yc6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string()
    )
    .is_empty())
}

#[test]
fn test_is_present() {
    assert_eq!(
        is_present(
            &connect(),
            "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string()
        ),
        false
    )
}

#[test]
fn test_event_persistent() {
    let puser: PUser = PUser {
        id: "f95dfd0b-e2fa-5b88-a284-578f9a015f4d".to_string(),
        name: "rahul".to_string(),
        email: "rsb007@gmail.com".to_string(),
        password: "rsb007@".to_string(),
    };
    let user_event: UserEvent = UserEvent::UserCreated(puser.clone());
    let user_state: UserState = UserState {
        user: puser,
        generation: 1,
    };
    assert_eq!(
        event_persistent(
            &connect(),
            &user_event,
            "f95dfd0b-e2fa-5b88-a284-578f9a015f4d".to_string(),
            &user_state
        ),
        Ok("successfully event stored")
    )
}
