use crate::controller::error::CustomError;
use crate::env_setup::connection::CurrentSession;
use crate::item_eventsourcing::item_event::models::ItemEvent;
use crate::item_eventsourcing::item_repository::display::select_item;
use crate::models::get_items::ItemMapper;
use crate::models::item::Item;
use crate::models::item_data::ItemData;
use crate::models::item_summary::ItemSummary;
use crate::models::p_item::PItem;
use actix_web::FromRequest;
use actix_web::Json;
use actix_web::Path;
use actix_web::State;
use actix_web::{HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::time::Duration;
use crate::models::item_status::PItemStatus;
use crate::models::item_data::PItemData;
use crate::constants::constants::ZERO;
///AppState is a struct with current session as field
pub struct AppState {
    pub session: CurrentSession,
    pub hashmap: HashMap<String, String>,
}

pub fn create_item(req: &HttpRequest<AppState>) -> Result<Item, CustomError> {
    unimplemented!();
}

pub fn get_item(data: State<AppState>, item_id: Path<String>) -> Result<PItem, CustomError> {
    let item_id: String = item_id.into_inner();
    let item_map = data.hashmap.to_owned();
    let item_mapper: Vec<ItemMapper> = select_item(&data.session, &item_id);
    // if item mapper len is greater that zero than item state exist
    if (item_mapper.len() == ZERO) {
        let item_state_in_cache:Option<&String> = item_map.get(&item_id);
        let item_not_exist = &"Not exist in cache".to_string();
        let state: &String = match item_state_in_cache {
            Some(state) => state,
            None => item_not_exist,
        };
        if (state == &"Not exist in cache".to_string()) {
            Err(CustomError::InvalidInput { field: "User not exist with this id" })
        }
        else { let pitem_state:PItem = serde_json::from_str(state).unwrap();
            Ok(pitem_state) }

    }
    else {
        Ok(PItem{
            id: 0,
            creator: "".to_string(),
            item_data: PItemData {
                title: "".to_string(),
                description: "".to_string(),
                currency_id: "".to_string(),
                increment: 0.0,
                reserve_price: 0.0,
                auction_duration: Duration::new(5,85),
                category_id: None
            },
            price: 0.0,
            status: PItemStatus::CREATED,
            auction_start: None,
            auction_end: None,
            auction_winner: None,
        })
    }
}

pub fn update_item(
    data: State<AppState>,
    item_id: Path<i32>,
    item_data: Json<ItemData>,
) -> Result<Item, CustomError> {
    unimplemented!();
}

pub fn start_auction(
    data: State<AppState>,
    item_id: Path<i32>,
) -> Result<&'static str, CustomError> {
    unimplemented!();
}

pub fn get_items_for_user(data: State<AppState>) -> Result<ItemSummary, CustomError> {
    unimplemented!();
}

/*
fn item_events() -> Topic<ItemEvent>{}*/
