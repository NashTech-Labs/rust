use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::p_user::PUser;

/// initial_state is used to set the initial value of UserState
pub fn initial_state() -> UserState {
    UserState {
        user: PUser {
            id: "".to_string(),
            name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        },
        generation: 0,
    }
}