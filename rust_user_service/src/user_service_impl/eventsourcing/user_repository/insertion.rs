use actix_web::Result;
use cdrs::query::QueryExecutor;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;

pub fn insert_user(session: &CurrentSession, new_user: UserRegistration)
                     -> Result<&'static str> {
    if is_present(&session, new_user.email)
        {   let user_json: String = serde_json::to_string(&new_user).unwrap();
            let insert_struct_cql = "INSERT INTO user_ks.user_events Json ?";
            session.query_with_values(insert_struct_cql,
                                      query_values!( user_json))
                .expect("insert error");
            Ok("welcome! user added")
        }
        else {
            Ok("user already exist")
        }
}