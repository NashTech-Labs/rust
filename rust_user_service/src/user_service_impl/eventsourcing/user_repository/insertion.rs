use actix_web::Result;
use cdrs::query::QueryExecutor;

use crate::user_service_impl::constants::queries::USER_EVENT_STORE_QUERY;
use crate::user_service_impl::constants::queries::USER_STATE_STORE_QUERY;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;

pub fn event_persistent(session: &CurrentSession, new_user: &UserEvent, user_id: String,
                        user_state: &UserState) -> Result<&'static str, CustomError> {
    let user_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(USER_EVENT_STORE_QUERY,
                           query_values!(user_id.clone(), user_json))
        .expect("insert error");
    let result: &str = match state_persistent(&session, &user_state, user_id) {
        Ok(_) => "User State Persisted",
        Err(_) => "Internal Error",
    };
    match result {
        "String" => Ok("successfully event stored"),
        _ => Err(CustomError::InternalError)
    }
}

fn state_persistent(session: &CurrentSession, new_user: &UserState, user_id: String)
                    -> Result<&'static str> {
    let user_state_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(
            USER_STATE_STORE_QUERY,
            query_values!(user_id, user_state_json),
        )
        .expect("insert error");
    Ok("successfully event stored")
}
