use std::cell::RefCell;

use actix_web::{Json,Result};
use actix_web::Path;
use actix_web::State;
use eventsourcing::Aggregate;
use uuid::Uuid;

use crate::user_service_impl::constants::constant::TAKE_FIRST;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::env_setup::keyspace::create_keyspace;
use crate::user_service_impl::env_setup::table::create_table;
use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_repository::display::select_all_user;
use crate::user_service_impl::eventsourcing::user_repository::display::select_user;
use crate::user_service_impl::eventsourcing::user_repository::insertion::event_persistent;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::get_user::UserMapper;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::models::user_login::UserLogin;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::utilities::initial_state::initial_state;
use crate::user_service_impl::utilities::mappers::map_user;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Error;
use crate::user_service_impl::utilities::wrapper::Outcomes;
use crate::user_service_impl::utilities::wrapper::wrap_vec;

///AppState is a struct with current session as field
pub struct AppState {
    pub session: CurrentSession,
}

/// initializer is used to create keyspace and tables
/// takes state which provide session for queries' execution
pub fn initializer(data: State<AppState>) -> Result<&'static str> {
    create_keyspace(&data.session);
    create_table(&data.session);
    Ok("environment successfully up")
}


/// create_user is a method which takes struct of UserRegistration and AppState
/// returns Result<Json<User>> in case of success and in case of failure,
/// it will return CustomError
/// create _user is used to storing the user details
pub fn create_user(data: State<AppState>, user_reg: Json<UserRegistration>)
                   -> Result<Json<User>, CustomError> {
    let new_user: UserRegistration = user_reg.into_inner();
    let new_user_id: String = get_id_by_email(&new_user.email).to_string();
    if is_present(&data.session, new_user_id.clone()) {
        let initial_user_state: UserState = initial_state();
        let create_user_command: UserCommand = UserCommand::CreateUser(new_user);
        let user_events: Vec<UserEvent> =
            PUser::handle_command(&initial_user_state, create_user_command)
                .unwrap();
        let user_state: UserState =
            PUser::apply_event(&initial_user_state, user_events[TAKE_FIRST]
                .clone()).unwrap();
        match event_persistent(&data.session, &user_events[TAKE_FIRST],
                               new_user_id, &user_state) {
            Ok(_) => Ok(Json(map_user(user_state.user))),
            Err(_) => Err(CustomError::InvalidInput {
                field: "Internal Server Error"
            }),
        }
    } else {
        Err(CustomError::InvalidInput {
            field: "user with this state already exist",
        })
    }
}

/// get_user is a method which takes user_id in its Path URL
/// returns Result<Json<User>> in case of success and in case of failure,
/// it will return CustomError
/// get_user is used to retrieve the user's details based on his/her user_id
pub fn get_user(data: State<AppState>, user_id: Path<String>)
                -> Result<Json<User>, CustomError> {
    let result: Vec<UserMapper> = select_user(&data.session, user_id.into_inner());
    if result.is_empty() {
        Err(CustomError::InvalidInput { field: "user with this id doesn't exist" })
    } else {
        let user_state: UserState = serde_json::
        from_str(&result[TAKE_FIRST].user_state).unwrap();
        Ok(Json(map_user(user_state.user)))
    }
}

///implementation for Outcomes
/// Outcomes is a struct which has vec<User> as its field
impl Responder for Outcomes {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

/// get_all_users is a method which takes shared state of current session
/// returns Responder
/// get_all_users is used to retrieve list of all user's details
pub fn get_all_users(req: &HttpRequest<AppState>) -> impl Responder {
    let user_mapper: Vec<UserMapper> = select_all_user(&req.state().session);
    let user_list: RefCell<Vec<User>> = RefCell::new(vec![]);
    if user_mapper.is_empty() {
        Err(CustomError::InternalError { field: "error in getting all users" })
    } else {
        for user in user_mapper {
            let user_state: UserState = serde_json::from_str(&user.user_state).unwrap();
            user_list.borrow_mut().push(map_user(user_state.user));
        }
        let vec_of_user: Vec<User> = user_list.borrow().to_vec();

        Ok(wrap_vec(vec_of_user))
    }
}

///this method is used to authenticate the user so that he can get his id
pub fn user_login(data: State<AppState>, user_login: Json<UserLogin>)
                  -> Result<String, CustomError> {
    let u_login: UserLogin = user_login.into_inner();
    let user_email: String = u_login.email;
    let user_id: String= get_id_by_email(&user_email).to_string();
    let user_status: Vec<UserMapper> = select_user(&data.session,
                                                   user_id.clone());
    if user_status.is_empty() {
        Err(CustomError::InvalidInput { field: "user not found" })
    } else {
        let user_state: UserState = serde_json::
        from_str(&user_status[TAKE_FIRST].user_state).unwrap();
        let user_password: String = user_state.user.password;
        if user_password == u_login.password {
            Ok(user_id)
        } else {
            Err(CustomError::InvalidInput {
                field: "username and password doesn't matched"
            })
        }
    }
}

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_email: &String) -> Uuid {
    let user_id: Uuid = Uuid::
    new_v5(&Uuid::NAMESPACE_URL, user_email.as_bytes());
    user_id
}
