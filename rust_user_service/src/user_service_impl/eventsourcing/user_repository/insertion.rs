use actix_web::{Json, Result, Path, App,HttpRequest};
use cdrs::query::QueryExecutor;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::controller::error::CustomError;
use uuid::Uuid;
use uuid::parser::ParseError;

pub fn insert_user(session: &CurrentSession, new_user: Json<UserRegistration>)
                     -> Result<Json<User>, CustomError> {
    let user_registration :UserRegistration = new_user.into_inner();
    let new_user_id = get_id_by_email(&user_registration).unwrap();
    if is_present(&session, new_user_id)
        {   let user_json: String = serde_json::to_string(&user_registration).unwrap();
            let insert_struct_cql = "INSERT INTO user_ks.user_events Json ?";
            session.query_with_values(insert_struct_cql,
                                      query_values!( user_json))
                .expect("insert error");
            Ok(Json(User{
                id: new_user_id.to_string(),
                name: user_registration.name,
                email: user_registration.email,
            }))
        }
        else {
            Err(CustomError::InvalidInput { field: "User State already exists" })
        }
}

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_reg:&UserRegistration)-> Result<Uuid,ParseError> {
    let id= &user_reg.email.to_lowercase();
    let user_id = Uuid::parse_str(id);
    user_id
}