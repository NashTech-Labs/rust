use actix_web::Result;
use cdrs::query::QueryExecutor;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;
use uuid::Uuid;
use uuid::parser::ParseError;
use actix_web::Json;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::controller::error::CustomError;

pub fn insert_user(session: &CurrentSession, new_user: UserRegistration)
                   -> Result<Json<User>, CustomError> {
    let new_user_id = get_id_by_email(&new_user).unwrap();
    if is_present(&session, new_user_id)
        {
            let user_json: String = serde_json::to_string(&new_user).unwrap();
            let insert_struct_cql = "INSERT INTO user_ks.user_events Json ?";
            session.query_with_values(insert_struct_cql,
                                      query_values!( user_json))
                .expect("insert error");
            Ok(Json(User {
                id: new_user_id.to_string(),
                name: new_user.name,
                email: new_user.email,
            }))
        } else {
        Err(CustomError::InvalidInput {field:"user with this state already exist"})
    }
}

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_reg: &UserRegistration) -> Result<Uuid, ParseError> {
    //let bytes= user_reg.email.to_lowercase().bytes();
    let user_id = Uuid::parse_str(&user_reg.email);
    user_id
}