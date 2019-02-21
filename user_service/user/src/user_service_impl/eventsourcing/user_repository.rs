use crate::error::CustomError;
use crate::user_service_impl::eventsourcing::user_event::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::UserState;
use actix_web::Result;
use cdrs::frame::Frame;
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::query::QueryExecutor;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};
use std::cell::RefCell;
use crate::db_connection::CurrentSession;
use crate::constants::{USER_STATE_STORE_QUERY, USER_EVENT_STORE_QUERY, SELECT_QUERY, SELECT_ALL_QUERY};
use futures::future::{ok,err};
use futures::Future;

/// UserMapper is used to map the details at retrieval time
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct UserMapper {
    pub user_id: String,
    pub user_state: String,
}

/// event_persistent is used to store the events against a particular user
pub fn event_persistent(
    session: &CurrentSession,
    new_user: &UserEvent,
    user_id: String,
    user_state: &UserState,
) -> impl Future<Item = &'static str, Error= CustomError> {
    let user_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(
            USER_EVENT_STORE_QUERY,
            query_values!(user_id.clone(), user_json),
        )
        .expect("insert error");
    state_persistent(&session, &user_state, user_id)
}

/// state_persistent is used to store the states against a particular user
fn state_persistent(
    session: &CurrentSession,
    new_user: &UserState,
    user_id: String,
) -> impl Future<Item = &'static str, Error= CustomError> {
    let user_state_json: String = serde_json::to_string(&new_user).unwrap();
    let query_status: Result<Frame, Error> = session.query_with_values(
        USER_STATE_STORE_QUERY,
        query_values!(user_id, user_state_json),
    );
    if query_status.is_ok() {
        ok("successfully state stored")
    } else {
        err(CustomError::InternalError {
            field: "error in state persistent",
        })
    }
}

/// select_user is used to retrieve a user detail based on user_id
pub fn get_user(session: &CurrentSession, user_id: String) -> impl Future<Item=Vec<UserMapper>, Error=()> {
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
    let user_mappers: Vec<UserMapper> = get_user_list.borrow().to_vec();
    ok(user_mappers)
}

/// select_all_user is used to retrieve list of all users' details
pub fn get_all_user(session: &CurrentSession) -> impl Future<Item=Vec<UserMapper>, Error=()> {
    let user_state_rows: Vec<Row> = session
        .query(SELECT_ALL_QUERY)
        .expect("is_select_all error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let get_users: RefCell<Vec<UserMapper>> = RefCell::new(vec![]);
    for row in user_state_rows {
        get_users
            .borrow_mut()
            .push(UserMapper::try_from_row(row).expect("into get user"));
    }
    let user_mappers: Vec<UserMapper> = get_users.borrow().to_vec();
    ok(user_mappers)
}

/// is_present is used to check whether a particular user's state is exists in database or not
pub fn is_present(session: &CurrentSession, id: String) -> impl Future<Item=bool, Error=()> {
    ok(session
        .query_with_values(SELECT_QUERY, query_values!(id))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty())
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
    use futures::future::Future;

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
        assert_eq!(
            state_persistent(
                &connect(),
                &user_state,
                "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
            ).wait(),
            Ok("successfully state stored")
        );
        session.query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'c6fd1799-b363-57f5-a4f5-6bfc12cef619'")
            .expect("Deletion error in test");
    }

    #[test]
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
            ).wait().unwrap(),
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
        assert_ne!(get_all_user(&connect()).wait().unwrap().len(), 0);
        session.query("DELETE from user_event_sourcing_ks.user_states WHERE user_id = 'c6fd1799-b363-57f5-a4f5-6bfc12cef619'")
            .expect("Deletion error in  Select_user test");
    }

    #[test]
    fn test_select_user_not_exist() {
        initializer(&connect());
        assert!(get_user(
            &connect(),
            "yc6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
        ).wait().unwrap()
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
            ).wait().unwrap(),
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
            ).wait(),
            Ok("successfully state stored")
        );
        session.query("DELETE from user_event_sourcing_ks.user_events WHERE user_id = 'f95dfd0b-e2fa-5b88-a284-578f9a015f4d'")
            .expect("Deletion error in event persistent test");
    }
}
