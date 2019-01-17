use std::cell::RefCell;

use cdrs::{self, types::prelude::*};
use cdrs::query::QueryExecutor;

use crate::item_service_impl::constants::queries::SELECT_QUERY;
use crate::item_service_impl::env_setup::connection::CurrentSession;
use crate::item_service_impl::models::get_items::ItemMapper;
use crate::item_service_impl::constants::queries::SELECT_ALL_QUERY;

pub fn select_item(session: &CurrentSession, item_id: &String) -> Vec<ItemMapper> {
    let item_state_rows: Vec<Row> = session
        .query_with_values(SELECT_QUERY, query_values!(item_id.to_owned()))
        .expect("is_select error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let get_item_list: RefCell<Vec<ItemMapper>> = RefCell::new(vec![]);
    for row in item_state_rows {
        get_item_list.borrow_mut().push(ItemMapper::try_from_row(row).expect("into get user"));
    }
    let result: Vec<ItemMapper> =get_item_list.borrow().to_vec();
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
/*

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
}*/
