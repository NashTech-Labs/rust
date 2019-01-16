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

/*///tail recursive call for retrieving user from user_state
fn convert(rows: Vec<Row>) -> Vec<GetUser> {
    let index: i32 = 0;
    let acc: RefCell<Vec<GetUser>> = RefCell::new(vec![]);
    fn sub_convert(rows: Vec<Row>, acc: RefCell<Vec<GetUser>>, index: i32) -> Rec<Vec<GetUser>>
    {
        let r_size: usize = rows.len();
        if r_size != 0 {
            let get_user: GetUser = GetUser::try_from_row(rows[index]).expect("into get user");
            acc.borrow_mut().push(get_user);
            rec_call!(sub_convert(rows, acc,index+1))
        } else {
            rec_ret!(acc.borrow().to_vec())
        }
    }
    tramp(sub_convert(rows, acc, index))
}*/

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