use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::constants::queries::SELECT_QUERY;
use cdrs::types::prelude::Row;
use crate::user_service_impl::constants::constant::TAKE_FIRST;
use crate::user_service_impl::models::get_user::GetUser;
use std::collections::vec_deque::Iter;

pub fn select_user(session: &CurrentSession, user_id: String) -> Result<User, CustomError> {
  unimplemented!()/*  let user_state:Vec<Row> = session
        .query_with_values(SELECT_QUERY, query_values!(user_id))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");
  let result:Vec<GetUser> = convert(user_state);
  let a= result[TAKE_FIRST];
    a.user_state.user*/
}
/*
fn convert(v: Vec<Row>) -> Vec<GetUser> {
    let user_vec:Vec<GetUser>;
    let v_iter = v.iter();
    fn mapping(a:Row) ->GetUser {
        GetUser::try_from_row(a).expect("mapping error")
    }
    mapping(v_iter.next())
}*/


pub fn select_all_user(session: &CurrentSession) -> Result<Vec<User>, CustomError> {
unimplemented!()
}