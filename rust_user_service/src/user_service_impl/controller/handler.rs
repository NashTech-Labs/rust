use actix_web::State;
use actix_web::{App, HttpRequest, Json, Path, Result};
use uuid::parser::ParseError;
use uuid::Uuid;

use crate::user_service_impl::constants::constant::TAKE_FIRST;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::{connect, CurrentSession};
use crate::user_service_impl::env_setup::keyspace::create_keyspace;
use crate::user_service_impl::env_setup::table::create_table;
use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_repository::insertion::event_persistent;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::models::user_login::UserLogin;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::utilities::initial_state::initial_state;
use crate::user_service_impl::utilities::mappers::user_mapper;

pub struct AppState {
    pub session: CurrentSession,
}

pub fn initializer(data: State<AppState>) -> Result<&'static str> {
    create_keyspace(&data.session);
    create_table(&data.session);
    Ok("successfully up")
}

pub fn create_user(
    data: State<AppState>,
    user_reg: Json<UserRegistration>,
) -> Result<Json<User>, CustomError> {
    let new_user: UserRegistration = user_reg.into_inner();
    let new_user_id: Uuid = get_id_by_email(&new_user).unwrap();

    if is_present(&data.session, new_user_id) {
        let initial_user_state: UserState = initial_state();
        let create_user_command: UserCommand = UserCommand::CreateUser(new_user);
        let user_events: Vec<UserEvent> =
            PUser::handle_command(&initial_user_state, create_user_command).unwrap();
        let user_state: UserState =
            PUser::apply_event(initial_user_state.clone(), &user_events[TAKE_FIRST]).unwrap();
        event_persistent(
            &data.session,
            &user_events[TAKE_FIRST],
            new_user_id,
            &user_state,
        );

        Ok(Json(user_mapper(user_state.user)))
    } else {
        Err(CustomError::InvalidInput {
            field: "user with this state already exist",
        })
    }
}

/*
pub fn get_user(user_id: Path<i32>) -> Result<Json<User>, CustomError> {
    select_user(&session,user_id.into_inner())
}

pub fn get_all_users() -> Result<Vec<User>, CustomError> {
    select_all_user(&session)
}

pub fn user_login(user_login: Json<UserLogin>) -> Result<&'static str, CustomError> {
    unimplemented!()
}
*/

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_reg: &UserRegistration) -> Result<Uuid, ParseError> {
    let user_id: Result<Uuid, ParseError> = Uuid::parse_str(&user_reg.email);
    user_id
}
