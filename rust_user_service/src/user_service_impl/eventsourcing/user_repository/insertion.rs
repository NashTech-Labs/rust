use actix_web::Json;
use actix_web::Result;
use cdrs::query::QueryExecutor;
use uuid::parser::ParseError;
use uuid::Uuid;

use crate::user_service_impl::constants::queries::USER_EVENT_STORE_QUERY;
use crate::user_service_impl::constants::queries::USER_STATE_STORE_QUERY;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::utilities::initial_state::initial_state;

pub fn event_persistent(
    session: &CurrentSession,
    new_user: &UserEvent,
    user_id: Uuid,
    user_state: &UserState,
) -> Result<&'static str> {
    let user_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(USER_EVENT_STORE_QUERY, query_values!(user_id, user_json))
        .expect("insert error");
    state_persistent(&session, &user_state, user_id);
    Ok("successfully event stored")
}
fn state_persistent(
    session: &CurrentSession,
    new_user: &UserState,
    user_id: Uuid,
) -> Result<&'static str> {
    let user_state_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(
            USER_STATE_STORE_QUERY,
            query_values!(user_id, user_state_json),
        )
        .expect("insert error");
    Ok("successfully event stored")
}
