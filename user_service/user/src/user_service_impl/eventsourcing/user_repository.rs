use crate::db_connection::CurrentSession;
use crate::error::CustomError;
use crate::model::User;
use crate::user_service_impl::env_setup::initializer;
use crate::user_service_impl::eventsourcing::user_entity::PUser;
use crate::user_service_impl::eventsourcing::user_event::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::UserState;
use actix_web::http;
use actix_web::AsyncResponder;
use actix_web::HttpResponse;
use actix_web::Json;
use actix_web::Result;
use cdrs::frame::Frame;
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::query::QueryExecutor;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};
use config::Config;
use config::ConfigError;
use futures::future::result;
use futures::Future;
use glob::glob;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use crate::utility::Outcomes;
use crate::utility::wrap_vec;

static INDEX: usize = 0;

/// UserMapper is used to map the details at retrieval time
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, TryFromRow)]
pub struct UserMapper {
    pub user_id: String,
    pub user_state: String,
}

pub fn configration_reader() -> HashMap<String, String> {
    let mut settings: Config = Config::default();
    settings
        .merge(config::File::with_name("config/Query"))
        .unwrap();
    settings.try_into::<HashMap<String, String>>().unwrap()
}

/// map_user is used to map PUser into User
pub fn map_user(user: PUser) -> User {
    User {
        id: user.id,
        name: user.name,
        email: user.email,
    }
}

/// event_persistent is used to store the events against a particular user
pub fn event_persistent(
    session: &CurrentSession,
    new_user: &UserEvent,
    user_id: String,
    user_state: UserState,
) -> Box<Future<Item = Json<User>, Error = CustomError>> {
    let USER_EVENT_STORE_QUERY = configration_reader()
        .get("user_event_store_query")
        .expect("event store query")
        .to_owned();

    let user_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(
            USER_EVENT_STORE_QUERY,
            query_values!(user_id.clone(), user_json),
        )
        .expect("insert error");
    state_persistent(&session, user_state, user_id)
}

/// state_persistent is used to store the states against a particular user
pub fn state_persistent<'a, 'b>(
    session: &'a CurrentSession,
    new_user: UserState,
    user_id: String,
) -> Box<Future<Item = Json<User>, Error = CustomError>> {
    let user_state_json: String = serde_json::to_string(&new_user).unwrap();

    let USER_STATE_STORE_QUERY = configration_reader()
        .get("user_state_store_query")
        .unwrap()
        .to_owned();

    let query_status: Result<Frame, Error> = session.query_with_values(
        USER_STATE_STORE_QUERY,
        query_values!(user_id, user_state_json),
    );
    if query_status.is_ok() {
        result(Ok(Json(map_user(new_user.user)))).responder()
    } else {
        result(Err(CustomError::InvalidInput {
            field: "Internal Server Error",
        }))
        .responder()
    }
}

/// select_user is used to retrieve a user detail based on user_id
pub fn get_user(
    session: &CurrentSession,
    user_id: String,
) -> Box<Future<Item = Json<User>, Error = CustomError>> {
    let SELECT_QUERY = configration_reader()
        .get("select_query")
        .unwrap()
        .to_owned();

    let user_state_rows: Vec<Row> = session
        .query_with_values(SELECT_QUERY, query_values!(user_id))
        .expect("is_select error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let get_user_list: RefCell<Vec<UserMapper>> = RefCell::new(vec![]);
    for row in user_state_rows {
        get_user_list
            .borrow_mut()
            .push(UserMapper::try_from_row(row).expect("into get user"));
    }
    let users: Vec<UserMapper> = get_user_list.borrow().to_vec();
    if users.is_empty() {
        result(Err(CustomError::InvalidInput {
            field: "user with this id doesn't exist",
        }))
        .responder()
    } else {
        let user_state: UserState = serde_json::from_str(&users[INDEX].user_state).unwrap();
        result(Ok(Json(map_user(user_state.user)))).responder()
    }
}

/// select_all_user is used to retrieve list of all users' details
pub fn get_all_user(
    session: &CurrentSession,
) -> Box<Future<Item = Json<Outcomes<User>>, Error = CustomError>> {
    let SELECT_ALL_QUERY = configration_reader()
        .get("select_all_query")
        .unwrap()
        .to_owned();
    let user_state_rows: Vec<Row> = session
        .query(SELECT_ALL_QUERY)
        .expect("is_select_all error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let get_user_list: RefCell<Vec<UserMapper>> = RefCell::new(vec![]);
    for row in user_state_rows {
        get_user_list
            .borrow_mut()
            .push(UserMapper::try_from_row(row).expect("into get user"));
    }
    let user_mapper: Vec<UserMapper> = get_user_list.borrow().to_vec();

    let users: RefCell<Vec<User>> = RefCell::new(vec![]);
    if user_mapper.is_empty() {
        result(Err(CustomError::InternalError {
            field: "error in getting all users",
        }))
        .responder()
    } else {
        for user in user_mapper {
            let user_state: UserState = serde_json::from_str(&user.user_state).unwrap();
            users.borrow_mut().push(map_user(user_state.user));
        }
        let user_list: Vec<User> = users.borrow().to_vec();

        result(Ok(Json(wrap_vec(user_list)))).responder()
    }
}

/// is_present is used to check whether a particular user's state is exists in database or not
pub fn is_present(session: &CurrentSession, id: String) -> bool {
    let SELECT_QUERY = configration_reader()
        .get("select_query")
        .unwrap()
        .to_owned();
    session
        .query_with_values(SELECT_QUERY, query_values!(id))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty()
}

pub fn check_user_exist(session: &CurrentSession, user_id: String) -> Vec<UserMapper> {
    let SELECT_QUERY = configration_reader()
        .get("select_query")
        .unwrap()
        .to_owned();

    let user_state_rows: Vec<Row> = session
        .query_with_values(SELECT_QUERY, query_values!(user_id.to_owned()))
        .expect("is_select error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let get_user_list: RefCell<Vec<UserMapper>> = RefCell::new(vec![]);
    for row in user_state_rows {
        get_user_list
            .borrow_mut()
            .push(UserMapper::try_from_row(row).expect("into get user"));
    }
    let user_status: Vec<UserMapper> = get_user_list.borrow().to_vec();
    user_status
}

#[cfg(test)]
mod tests {
    use crate::db_connection::connect;
    use crate::user_service_impl::eventsourcing::user_state::UserState;
    use crate::user_service_impl::eventsourcing::user_repository::state_persistent;
    use crate::user_service_impl::eventsourcing::user_repository::UserMapper;
    use crate::user_service_impl::eventsourcing::user_repository::get_user;
    use crate::user_service_impl::eventsourcing::user_repository::get_all_user;
    use crate::user_service_impl::eventsourcing::user_repository::is_present;
    use crate::user_service_impl::eventsourcing::user_entity::PUser;
    use crate::user_service_impl::eventsourcing::user_event::UserEvent;
    use crate::user_service_impl::eventsourcing::user_repository::event_persistent;
    use crate::user_service_impl::env_setup::initializer;
    use crate::db_connection::CurrentSession;
    use cdrs::query::QueryExecutor;
    use crate:: model::User;
    use crate:: user_service_impl::eventsourcing::user_repository::map_user;
    use actix_web::Json;
    //use futures::future::result;
    //use actix_web::AsyncResponder;
    //use futures_timer::FutureExt;
   // use futures::prelude::*;
    use futures::Future;
    use futures::sync::oneshot;
    use std::time::Duration;
    use actix_web::error::ParseError::Timeout;
    use tokio::timer::Timeout;
    #[test]
    fn test_state_persistent() {
        let session: CurrentSession = connect();
        initializer(&session);
        let user_state: UserState = UserState {
            user: PUser {
                id: "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
                name: "shikha".to_string(),
                email: "shikha97887@gmail.com".to_string(),
                password: "shikha123".to_string(),
            },
            generation: 1,
        };
        let user_state_copy = user_state.to_owned();

        let user_state_status = Timeout::new(state_persistent(
            &connect(),
            user_state,
            "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
        ),Duration::from_secs(10)).into_inner();

        let b = user_state_status.wait();

        assert_eq!(user_state_copy,
            Json(map_user(user_state_copy.user))
        );
        session.query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'c6fd1799-b363-57f5-a4f5-6bfc12cef619'")
            .expect("Deletion error in test");
    }

   /* #[test]
    fn test_get_user() {
        let session: CurrentSession = connect();
        initializer(&session);
        let user_mapper: UserMapper = UserMapper {
            user_id: "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
            user_state:
            "{\"user\":{\"id\":\"c6fd1799-b363-57f5-a4f5-6bfc12cef619\",\"name\":\"shikha\",\
             \"email\":\"shikha97887@gmail.com\",\"password\":\"shikha123\"},\"generation\":1}"
                .to_string(),
        };
        let user_detail: Vec<UserMapper> = vec![user_mapper.clone()];
        session.query_with_values("INSERT INTO user_event_sourcing_ks.user_states (user_id,user_state) \
     VALUES (?,?)", query_values!(user_mapper.user_id,user_mapper.user_state))
            .expect("Insert Error in Select_user test");


        assert_eq!(
            get_user(
                &connect(),
                "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
            ),
            user_detail
        );
        session.query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'c6fd1799-b363-57f5-a4f5-6bfc12cef619'")
            .expect("Deletion error in  Select_user test");
    }

    #[test]
    fn test_get_all_user() {
        let session: CurrentSession = connect();
        initializer(&session);
        let user_mapper: UserMapper = UserMapper {
            user_id: "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
            user_state:
            "{\"user\":{\"id\":\"c6fd1799-b363-57f5-a4f5-6bfc12cef619\",\"name\":\"shikha\",\
             \"email\":\"shikha97887@gmail.com\",\"password\":\"shikha123\"},\"generation\":1}"
                .to_string(),
        };
        session.query_with_values("INSERT INTO user_event_sourcing_ks.user_states (user_id,user_state) \
     VALUES (?,?)", query_values!(user_mapper.user_id,user_mapper.user_state))
            .expect("Insert Error in Select_user test");
        assert_ne!(get_all_user(&connect()).len(), 0);
        session.query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'c6fd1799-b363-57f5-a4f5-6bfc12cef619'")
            .expect("Deletion error in  Select_user test");
    }

    #[test]
    fn test_select_user_not_exist() {
        initializer(&connect());
        assert!(get_user(
            &connect(),
            "yc6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
        )
            .is_empty())
    }

    #[test]
    fn test_is_present() {
        let session: CurrentSession = connect();
        initializer(&session);
        assert_eq!(
            is_present(
                &session,
                "f95dfd0b-e2fa-5b88-a284-578f9a015f4d".to_string(),
            ),
            false
        )
    }

    #[test]
    fn test_event_persistent() {
        let session: CurrentSession = connect();
        initializer(&session);
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
                &user_state,
            ),
            Ok("successfully event stored")
        );
        session.query("DELETE from user_event_sourcing_ks.user_events WHERE user_id = 'f95dfd0b-e2fa-5b88-a284-578f9a015f4d'")
            .expect("Deletion error in event persistent test");
    }*/
}
