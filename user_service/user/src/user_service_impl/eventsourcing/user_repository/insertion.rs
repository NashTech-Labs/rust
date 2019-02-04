use actix_web::http;
use actix_web::HttpResponse;
use actix_web::Result;
use cdrs::frame::Frame;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::Error;

use crate::user_service_impl::constants::queries::USER_EVENT_STORE_QUERY;
use crate::user_service_impl::constants::queries::USER_STATE_STORE_QUERY;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;

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
        Err(CustomError::InternalError { field: "error in event persistent" })
    }
}

/// state_persistent is used to store the states against a particular user
fn state_persistent<'a, 'b>(
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
        Err(CustomError::InternalError { field: "error in state persistent" })
    }
}

#[test]
fn test_state_persistent() {
    use crate::user_service_impl::env_setup::connection::connect;
    let user_state: UserState = UserState{ user: puser, generation: 1 };
    assert_eq!(state_persistent(&connect(),&user_state,"f95dfd0b-e2fa-5b88-a284-578f9a015f4d"
        .to_string()),Ok("successfully state stored"))
}

