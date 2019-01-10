use actix_web::Json;
use actix_web::Path;
use crate::controller::error::CustomError;
use crate::models::item_data::ItemData;
use crate::models::item::Item;
use crate::models::item_summary::ItemSummary;
use crate::eventsourcing::item_event::models::ItemEvent;

pub fn create_item(item_data: Json<ItemData>) -> Result<Item,CustomError> {
    unimplemented!();
}

pub fn get_item(item_id:Path<id>) -> Result<Item,CustomError> {
    unimplemented!();
}

pub fn update_item(item_id:Path<id>,item_data: Json<ItemData>) -> Result<Item,CustomError> {
    unimplemented!();
}

pub fn start_auction(item_id:Path<id>) -> &Result<&str,CustomError> {
    unimplemented!();
}

pub fn get_items_for_user() -> Result<ItemSummary,CustomError> {
    unimplemented!();
}

fn item_events() -> Topic<ItemEvent>{}