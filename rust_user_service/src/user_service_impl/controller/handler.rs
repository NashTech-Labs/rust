use actix_web::{Json, Result, Path, App,HttpRequest};
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::models::user_login::UserLogin;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::{CurrentSession,connect};
use crate::user_service_impl::eventsourcing::user_repository::insertion::insert_user;
use std::cell::Cell;
use actix_web::State;
use crate::user_service_impl::env_setup::keyspace::create_keyspace;
use crate::user_service_impl::env_setup::table::create_table;

pub struct AppState {
    pub session: CurrentSession,
}

pub fn intializer(data:State<AppState>)-> Result<&'static str> {
    create_keyspace(&data.session);
    create_table(&data.session);
    Ok("successfully up")
}
pub fn create_user(data:State<AppState>,user_reg: Json<UserRegistration>) -> Result<Json<User>, CustomError> {
    insert_user(&data.session,user_reg.into_inner())
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
