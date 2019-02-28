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
use crate::user_service_impl::eventsourcing::user_repository::is_present;
use crate::user_service_impl::eventsourcing::user_repository::get_all_user;
use crate::user_service_impl::eventsourcing::user_repository::get_user;
use crate::user_service_impl::eventsourcing::user_repository::UserMapper;
use crate::user_service_impl::eventsourcing::user_state::UserState;
use crate::utility::wrap_vec;
use crate::utility::Outcomes;
use actix_web::*;
use eventsourcing::Aggregate;
use futures::future::result;
use futures::Future;
use std::cell::RefCell;
use uuid::Uuid;
use std::error::Error;
use actix_web::middleware::session::{RequestSession, SessionStorage, CookieSessionBackend};
use actix_web::middleware::session::Session;
use validator::{Validate, ValidationError};

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
        session: Session,
    ) -> Box<Future<Item=Json<User>, Error=CustomError>> {
        let new_user: UserRegistration = user_reg.into_inner();
        match new_user.validate() {
            Ok(_) => {
                let new_user_id: String = get_id_by_email(new_user.email.as_str()).to_string();
                if is_present(&data.session, new_user_id.clone()) {
                        let initial_user_state: UserState = initial_state();
                        let user_id:String = get_id_by_email(&initial_user_state.user.email).to_string();

                        session.set("userid",user_id);

                        let create_user_command: UserCommand = UserCommand::CreateUser(new_user);
                        let user_events: Vec<UserEvent> =
                            PUser::handle_command(&initial_user_state, create_user_command).unwrap();
                        let user_state: UserState =
                            PUser::apply_event(&initial_user_state, user_events[INDEX].clone()).unwrap();
                        match event_persistent(&data.session, &user_events[INDEX], new_user_id, &user_state) {
                            Ok(_) => result(Ok(Json(map_user(user_state.user)))).responder(),
                            Err(_) => result(Err(CustomError::InvalidInput {
                                field: "Internal Server Error",
                            }))
                                .responder(),
                        }


                } else {
                    result(Err(CustomError::InvalidInput {
                        field: "user with this state already exist",
                    }))
                        .responder()
                }
            }
            Err(_) => {
                result(Err(CustomError::ValidationError {
                    field: "Invalid Input: 1.Name cannot be null 2.Email must be valid 3. Password must be atleast 6 character",
                }))
                    .responder()
            }
        }
    }

    /// get_user is a method which takes user_id in its Path URL
    /// returns Result<Json<User>> in case of success and in case of failure,
    /// it will return CustomError
    /// get_user is used to retrieve the user's details based on his/her user_id
    fn get_user(
        data: State<AppState>,
        user_id: Path<String>,
        session: Session,
    ) -> Box<Future<Item=Json<User>, Error=CustomError>> {
        let user_mapper_list: Vec<UserMapper> = get_user(&data.session, user_id.into_inner());
        if user_mapper_list.is_empty() {
            result(Err(CustomError::InvalidInput {
                field: "user with this id doesn't exist",
            }))
                .responder()
        } else {
            if let Some(userid) = session.get::<String>("userid").unwrap() {
                println!("SESSION value: {}", userid);
                let user_state: UserState =
                    serde_json::from_str(&user_mapper_list[INDEX].user_state).unwrap();
                result(Ok(Json(map_user(user_state.user)))).responder()
            } else {

               result(Err(CustomError::InvalidInput {field : "Please sign in"})).responder()
            }
        }
    }

    /// get_all_users is a method which takes shared state of current session
    /// returns Responder
    /// get_all_users is used to retrieve list of all user's details
    fn get_all_users(
        data: State<AppState>,
        session: Session,
    ) -> Box<Future<Item=Json<Outcomes<User>>, Error=CustomError>> {
        let user_mapper: Vec<UserMapper> = get_all_user(&data.session);
        let user_list: RefCell<Vec<User>> = RefCell::new(vec![]);
        if user_mapper.is_empty() {
            result(Err(CustomError::InternalError {
                field: "error in getting all users",
            }))
                .responder()
        } else {

            if let Some(userid) = session.get::<String>("userid").unwrap() {
                for user in user_mapper {
                    let user_state: UserState = serde_json::from_str(&user.user_state).unwrap();
                    user_list.borrow_mut().push(map_user(user_state.user));
                }
                let vec_of_user: Vec<User> = user_list.borrow().to_vec();

                result(Ok(Json(wrap_vec(vec_of_user)))).responder()
            } else {
                result(Err(CustomError::InvalidInput {field : "Please sign in"})).responder()
            }
        }
    }

    ///this method is used to authenticate the user so that he can get his id
    fn user_login(
        data: State<AppState>,
        user_login: Json<UserLogin>,
        session: Session,
    ) -> Box<Future<Item=String, Error=CustomError>> {
        let u_login: UserLogin = user_login.into_inner();
        match u_login.validate() {
            Ok(_) => {
                let user_email: String = u_login.email;
                let user_id: String = get_id_by_email(user_email.as_str()).to_string();
                let user_status: Vec<UserMapper> = get_user(&data.session, user_id.clone());
                if user_status.is_empty() {
                    result(Err(CustomError::InvalidInput {
                        field: "user not found",
                    }))
                        .responder()
                } else {

                  //  session.set("userid", user_id.to_owned()).unwrap();
                 //   println!("SESSION value: {}", userid);
                    if let Some(userid) = session.get::<String>("userid").unwrap() {
                        println!("SESSION value: {}", userid);
                        session.set("userid", user_id.to_owned()).unwrap();
                    } else {
                        session.set("userid", user_id.to_owned()).unwrap();
                        println!("SESSION value-----------: {:?}", session.get::<String>("userid").unwrap());
                    }
                       //  println!("{:?}",session.try_into());

                   /* println!("hello");
                    if let Some(count) = session.get::<i32>("counter").unwrap() {
                        println!("SESSION value: {}", count);
                        session.set("counter", count+1).unwrap();
                    } else {
                        session.set("counter", 1).unwrap();
                        println!("hi");
                    }*/

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
            Err(_) => {
                result(Err(CustomError::ValidationError {
                    field: "Invalid Input: 1.Email must be valid 2. Password must be atleast 6 character",
                }))
                    .responder()
            }
        }
    }
}

/*if let Some(count) = session.get::<i32>("counter").unwrap() {
                  println!("SESSION value: {}", count);
                 session.set("counter", count+1).unwrap();
              } else {
                    session.set("counter", 1).unwrap();
             }*/

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_email: &str) -> Uuid {
    let user_id: Uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, user_email.as_bytes());
    user_id
}

/// map_user is used to map PUser into User
pub fn map_user(user: PUser) -> User {
    User {
        id: user.id,
        name: user.name,
        email: user.email,
    }
}

#[cfg(test)]
mod tests {
    use crate::user_service_impl::handler::get_id_by_email;
    use crate::user_service_impl::handler::map_user;
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
                password: "qwerty".to_string(),
            }),
            User {
                id: "52ec207c-c87e-519e-9297-0c67cc2df8ee".to_string(),
                name: "Amita".to_string(),
                email: "amita.yadav@knoldus.in".to_string(),
            }
        )
    }
}
