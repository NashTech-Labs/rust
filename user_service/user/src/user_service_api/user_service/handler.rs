use std::cell::RefCell;

use actix_web::*;
use eventsourcing::Aggregate;
use uuid::Uuid;

use crate::user_service_impl::constants::constant::INDEX;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_repository::display::select_all_user;
use crate::user_service_impl::eventsourcing::user_repository::display::select_user;
use crate::user_service_impl::eventsourcing::user_repository::insertion::event_persistent;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::get_user::UserMapper;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::utilities::initial_state::initial_state;
use crate::user_service_impl::utilities::mappers::map_user;
use crate::user_service_impl::utilities::wrapper::Outcomes;
use crate::user_service_impl::utilities::wrapper::wrap_vec;
use crate::user_service_api::models::user::User;
use crate::user_service_api::models::user_registration::UserRegistration;
use crate::user_service_api::models::user_login::UserLogin;
use futures::Future;
use futures::future::result;


///AppState is a struct with current session as field
pub struct AppState {
    pub session: CurrentSession,
}

/// create_user is a method which takes struct of UserRegistration and AppState
/// returns Result<Json<User>> in case of success and in case of failure,
/// it will return CustomError
/// create _user is used to storing the user details
pub fn create_user(data: State<AppState>, user_reg: Json<UserRegistration>)
                   -> Box<Future<Item = Json<User>, Error = CustomError>> {
    let new_user: UserRegistration = user_reg.into_inner();
    let new_user_id: String = get_id_by_email(new_user.email.as_str()).to_string();
    if is_present(&data.session, new_user_id.clone()) {
        let initial_user_state: UserState = initial_state();
        let create_user_command: UserCommand = UserCommand::CreateUser(new_user);
        let user_events: Vec<UserEvent> =
            PUser::handle_command(&initial_user_state, create_user_command)
                .unwrap();
        let user_state: UserState =
            PUser::apply_event(&initial_user_state, user_events[INDEX]
                .clone()).unwrap();
        match event_persistent(&data.session, &user_events[INDEX],
                               new_user_id, &user_state) {
            Ok(_) => result(Ok(Json(map_user(user_state.user)))).responder(),
            Err(_) => result(Err(CustomError::InvalidInput {
                field: "Internal Server Error"
            })).responder(),
        }
    } else {
        result(Err(CustomError::InvalidInput {
            field: "user with this state already exist",
        })).responder()
    }
}

/// get_user is a method which takes user_id in its Path URL
/// returns Result<Json<User>> in case of success and in case of failure,
/// it will return CustomError
/// get_user is used to retrieve the user's details based on his/her user_id
pub fn get_user(data: State<AppState>, user_id: Path<String>)
                -> Box<Future<Item = Json<User>, Error = CustomError>> {
    let user_mapper_list: Vec<UserMapper> = select_user(&data.session, user_id.into_inner());
    if user_mapper_list.is_empty() {
        result(Err(CustomError::InvalidInput { field: "user with this id doesn't exist" })).responder()
    } else {
        let user_state: UserState = serde_json::
        from_str(&user_mapper_list[INDEX].user_state).unwrap();
        result(Ok(Json(map_user(user_state.user)))).responder()
    }
}

/// get_all_users is a method which takes shared state of current session
/// returns Responder
/// get_all_users is used to retrieve list of all user's details
pub fn get_all_users(data: State<AppState>) -> Box<Future<Item = Json<Outcomes>, Error = CustomError>> {
    let user_mapper: Vec<UserMapper> = select_all_user(&data.session);
    let user_list: RefCell<Vec<User>> = RefCell::new(vec![]);
    if user_mapper.is_empty() {
        result(Err(CustomError::InternalError { field: "error in getting all users" })).responder()
    } else {
        for user in user_mapper {
            let user_state: UserState = serde_json::from_str(&user.user_state).unwrap();
            user_list.borrow_mut().push(map_user(user_state.user));
        }
        let vec_of_user: Vec<User> = user_list.borrow().to_vec();

        result(Ok(Json(wrap_vec(vec_of_user)))).responder()
    }
}

///this method is used to authenticate the user so that he can get his id
pub fn user_login(data: State<AppState>, user_login: Json<UserLogin>)
                  -> Box<Future<Item = String, Error = CustomError>> {
    let u_login: UserLogin = user_login.into_inner();
    let user_email: String = u_login.email;
    let user_id: String = get_id_by_email(user_email.as_str()).to_string();
    let user_status: Vec<UserMapper> = select_user(&data.session,
                                                   user_id.clone());
    if user_status.is_empty() {
        result(Err(CustomError::InvalidInput { field: "user not found" })).responder()
    } else {
        let user_state: UserState = serde_json::
        from_str(&user_status[INDEX].user_state).unwrap();
        let user_password: String = user_state.user.password;
        if user_password == u_login.password {
            result(Ok(user_id)).responder()
        } else {
            result(Err(CustomError::InvalidInput {
                field: "username and password doesn't matched"
            })).responder()
        }
    }
}

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_email: &str) -> Uuid {
    let user_id: Uuid = Uuid::
    new_v5(&Uuid::NAMESPACE_URL, user_email.as_bytes());
    user_id
}