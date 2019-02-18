use crate::error::CustomError;
use crate::user_service_impl::eventsourcing::user_event::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::UserState;
use actix_web::http;
use actix_web::HttpResponse;
use actix_web::Result;
use cdrs::frame::Frame;
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::query::QueryExecutor;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};
use std::cell::RefCell;
use crate::db_connection::CurrentSession;

static USER_EVENT_STORE_QUERY: &str =
    "INSERT INTO user_event_sourcing_ks.user_events (user_id,user_event) \
     VALUES (?,?)";

static USER_STATE_STORE_QUERY: &str =
    "INSERT INTO user_event_sourcing_ks.user_states (user_id,user_state) \
     VALUES (?,?)";

static SELECT_QUERY: &str =
    "SELECT * FROM user_event_sourcing_ks.user_states WHERE user_id = ? ";

static SELECT_ALL_QUERY: &str = "SELECT * FROM user_event_sourcing_ks.user_states";

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
) -> Result<&'static str, CustomError> {
    let user_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(
            USER_EVENT_STORE_QUERY,
            query_values!(user_id.clone(), user_json),
        )
        .expect("insert error");
    let status: HttpResponse = match state_persistent(&session, &user_state, user_id) {
        Ok(_) => HttpResponse::new(http::StatusCode::OK),
        Err(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
    };
    if status.status().is_success() {
        Ok("successfully event stored")
    } else {
        Err(CustomError::InternalError {
            field: "error in event persistent",
        })
    }
}

/// state_persistent is used to store the states against a particular user
pub fn state_persistent<'a, 'b>(
    session: &'a CurrentSession,
    new_user: &'b UserState,
    user_id: String,
) -> Result<&'static str, CustomError> {
    let user_state_json: String = serde_json::to_string(&new_user).unwrap();
    let query_status: Result<Frame, Error> = session.query_with_values(
        USER_STATE_STORE_QUERY,
        query_values!(user_id, user_state_json),
    );
    if query_status.is_ok() {
        Ok("successfully state stored")
    } else {
        Err(CustomError::InternalError {
            field: "error in state persistent",
        })
    }
}

/// select_user is used to retrieve a user detail based on user_id
pub fn get_user(session: &CurrentSession, user_id: String) -> Vec<UserMapper> {
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
    let result: Vec<UserMapper> = get_user_list.borrow().to_vec();
    result
}

/// select_all_user is used to retrieve list of all users' details
pub fn get_all_user(session: &CurrentSession) -> Vec<UserMapper> {
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
    let result: Vec<UserMapper> = get_user_list.borrow().to_vec();
    result
}

/// is_present is used to check whether a particular user's state is exists in database or not
pub fn is_present(session: &CurrentSession, id: String) -> bool {
    session
        .query_with_values(SELECT_QUERY, query_values!(id))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty()
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

    #[test]
    fn test_state_persistent() {
       let session: CurrentSession= connect();
        initializer(&session);
        let user_state: UserState = UserState {
            user: PUser{
                id: "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string(),
                name: "shikha".to_string(),
                email: "shikha97887@gmail.com".to_string(),
                password: "shikha123".to_string()
            },
            generation: 1,
        };
        assert_eq!(
            state_persistent(
                &connect(),
                &user_state,
                "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string()
            ),
            Ok("successfully state stored")
        );
        session.query("DELETE from user_eventsourcing.user_state WHERE user_id = 'c6fd1799-b363-57f5-a4f5-6bfc12cef619'")
            .expect("Deletion error in test");
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
            get_user(
                &connect(),
                "c6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string()
            ),
            user_detail
        )
    }

    #[test]
    fn test_select_all_user() {
        assert_ne!(get_all_user(&connect()).len(), 0)
    }

    #[test]
    fn test_select_user_not_exist() {
        initializer(&connect());
        assert!(get_user(
            &connect(),
            "yc6fd1799-b363-57f5-a4f5-6bfc12cef619".to_string()
        )
            .is_empty())
    }

    #[test]
    fn test_is_present() {
        initializer(&connect());
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
        initializer(&connect());
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

}
