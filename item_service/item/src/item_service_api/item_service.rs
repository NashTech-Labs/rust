use actix_web::Json;
use actix_web::Path;
use actix_web::State;
use actix_web::FromRequest;
use crate::controller::error::CustomError;
use crate::models::item_data::ItemData;
use crate::models::item::Item;
use crate::models::item_summary::ItemSummary;
use crate::item_eventsourcing::item_event::models::ItemEvent;
use actix_web::{HttpRequest,HttpResponse};
use crate::env_setup::connection::CurrentSession;

///AppState is a struct with current session as field
pub struct AppState {
    pub session: CurrentSession,
}

pub fn create_item(req: &HttpRequest<AppState>) -> Result<Item,CustomError> {
    unimplemented!();

}

pub fn get_item(data: State<AppState>,item_id:Path<i32>) -> Result<Item,CustomError> {
    unimplemented!();
}

pub fn update_item(data: State<AppState>,item_id:Path<i32>,item_data: Json<ItemData>) -> Result<Item,CustomError> {
    unimplemented!();
}

pub fn start_auction(data: State<AppState>,item_id:Path<i32>) -> Result<&'static str,CustomError> {
    unimplemented!();
}

pub fn get_items_for_user(data: State<AppState>) -> Result<ItemSummary,CustomError> {
    unimplemented!();
}

/*
fn item_events() -> Topic<ItemEvent>{}*/
