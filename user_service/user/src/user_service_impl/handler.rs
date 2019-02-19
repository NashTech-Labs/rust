use crate::error::CustomError;
use crate::model::User;
use crate::model::UserLogin;
use crate::model::UserRegistration;
use crate::user_service_api::user_service::AppState;
use crate::user_service_api::user_service::UserService;
use crate::user_service_impl::eventsourcing::user_command::UserCommand;
use crate::user_service_impl::eventsourcing::user_entity::initial_state;
use crate::user_service_impl::eventsourcing::user_entity::PUser;
use crate::user_service_impl::eventsourcing::user_event::UserEvent;
use crate::user_service_impl::eventsourcing::user_repository::event_persistent;
use crate::user_service_impl::eventsourcing::user_repository::get_all_user;
use crate::user_service_impl::eventsourcing::user_repository::get_user;
use crate::user_service_impl::eventsourcing::user_repository::is_present;
use crate::user_service_impl::eventsourcing::user_repository::UserMapper;
use crate::user_service_impl::eventsourcing::user_state::UserState;

use crate::user_service_impl::eventsourcing::user_repository::check_user_exist;
use crate::utility::Outcomes;
//use crate::db_connection::CurrentSession;
use actix_web::*;
use cdrs::query::QueryExecutor;
use cdrs::{self, types::prelude::*};
use eventsourcing::Aggregate;
use futures::future::result;
use futures::Future;
use std::cell::RefCell;
use uuid::Uuid;

static INDEX: usize = 0;

pub struct UserInfo;
impl UserService for UserInfo {
    /// create_user is a method which takes struct of UserRegistration and AppState
    /// returns Result<Json<User>> in case of success and in case of failure,
    /// it will return CustomError
    /// create _user is used to storing the user details
    fn create_user(
        data: State<AppState>,
        user_reg: Json<UserRegistration>,
    ) -> Box<Future<Item = Json<User>, Error = CustomError>> {
        let new_user: UserRegistration = user_reg.into_inner();
        let new_user_id: String = get_id_by_email(new_user.email.as_str()).to_string();
        if is_present(&data.session, new_user_id.clone()) {
            let initial_user_state: UserState = initial_state();
            let create_user_command: UserCommand = UserCommand::CreateUser(new_user);
            let user_events: Vec<UserEvent> =
                PUser::handle_command(&initial_user_state, create_user_command).unwrap();
            let user_state: UserState =
                PUser::apply_event(&initial_user_state, user_events[INDEX].clone()).unwrap();
            event_persistent(&data.session, &user_events[INDEX], new_user_id, user_state)
        } else {
            result(Err(CustomError::InvalidInput {
                field: "user with this state already exist",
            }))
            .responder()
        }
    }

    /// get_user is a method which takes user_id in its Path URL
    /// returns Result<Json<User>> in case of success and in case of failure,
    /// it will return CustomError
    /// get_user is used to retrieve the user's details based on his/her user_id
    fn get_user(
        data: State<AppState>,
        user_id: Path<String>,
    ) -> Box<Future<Item = Json<User>, Error = CustomError>> {
        /* let user_mapper_list: Vec<UserMapper> = */
        get_user(&data.session, user_id.into_inner())

    }

    /// get_all_users is a method which takes shared state of current session
    /// returns Responder
    /// get_all_users is used to retrieve list of all user's details
    fn get_all_users(
        data: State<AppState>,
    ) -> Box<Future<Item = Json<Outcomes<User>>, Error = CustomError>> {
        get_all_user(&data.session)
    }

    ///this method is used to authenticate the user so that he can get his id
    fn user_login(
        data: State<AppState>,
        user_login: Json<UserLogin>,
    ) -> Box<Future<Item = String, Error = CustomError>> {
        let u_login: UserLogin = user_login.into_inner();
        let user_email: String = u_login.email;
        let user_id: String = get_id_by_email(user_email.as_str()).to_string();

        let user_status: Vec<UserMapper> = check_user_exist(&data.session, user_id.clone());

        if user_status.is_empty() {
            result(Err(CustomError::InvalidInput {
                field: "user not found",
            }))
            .responder()
        } else {
            let user_state: UserState =
                serde_json::from_str(&user_status[INDEX].user_state).unwrap();
            let user_password: String = user_state.user.password;
            if user_password == u_login.password {
                result(Ok(user_id)).responder()
            } else {
                result(Err(CustomError::InvalidInput {
                    field: "username and password doesn't matched",
                }))
                .responder()
            }
        }
    }
}

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_email: &str) -> Uuid {
    let user_id: Uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, user_email.as_bytes());
    user_id
}

#[cfg(test)]
mod tests {
    use crate::user_service_impl::handler::get_id_by_email;
    use crate::user_service_impl::eventsourcing::user_repository::map_user;
    use crate::user_service_impl::eventsourcing::user_entity::PUser;
    use crate::model::User;

    #[test]
    fn test_get_id_by_email() {
        assert_eq!(
            get_id_by_email("sid@gmail.com").to_string(),
            "a9c8536e-75ee-582b-a145-b6ace45abe9d".to_string()
        )
    }

    #[test]
    fn test_map_user() {
        assert_eq!(
            map_user(PUser {
                id: "52ec207c-c87e-519e-9297-0c67cc2df8ee".to_string(),
                name: "Amita".to_string(),
                email: "amita.yadav@knoldus.in".to_string(),
                password: "qwerty".to_string()
            }),
            User {
                id: "52ec207c-c87e-519e-9297-0c67cc2df8ee".to_string(),
                name: "Amita".to_string(),
                email: "amita.yadav@knoldus.in".to_string(),
            }
        )
    }
}
