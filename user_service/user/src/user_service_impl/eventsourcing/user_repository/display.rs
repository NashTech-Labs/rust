use std::cell::RefCell;

use cdrs::{self, types::prelude::*};
use cdrs::query::QueryExecutor;

use crate::user_service_impl::constants::queries::SELECT_QUERY;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::models::get_user::UserMapper;
use crate::user_service_impl::constants::queries::SELECT_ALL_QUERY;

/// select_user is used to retrieve a user detail based on user_id
pub fn select_user(session: &CurrentSession, user_id: String) -> Vec<UserMapper> {
    let user_state_rows: Vec<Row> = session
        .query_with_values(SELECT_QUERY, query_values!(user_id))
        .expect("is_select error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let get_user_list: RefCell<Vec<UserMapper>> = RefCell::new(vec![]);
    for row in user_state_rows {
        get_user_list.borrow_mut().push(UserMapper::try_from_row(row).expect("into get user"));
    }
    let result: Vec<UserMapper> =get_user_list.borrow().to_vec();
    result
}

/// select_all_user is used to retrieve list of all users' details
pub fn select_all_user(session: &CurrentSession) -> Vec<UserMapper> {
    let user_state_rows: Vec<Row> = session
        .query(SELECT_ALL_QUERY)
        .expect("is_select_all error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let get_user_list: RefCell<Vec<UserMapper>> = RefCell::new(vec![]);
    for row in user_state_rows {
        get_user_list.borrow_mut().push(UserMapper::try_from_row(row).expect("into get user"));
    }
    let result: Vec<UserMapper> =get_user_list.borrow().to_vec();
    result
}