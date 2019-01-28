use actix_web::{HttpRequest,HttpResponse,Json,Path,State,FromRequest};
use crate::item_service_impl::controller::error::CustomError;
use crate::item_service_impl::env_setup::connection::CurrentSession;
use crate::item_service_api::item::Item;
use crate::item_service_api::item_summary::ItemSummary;
use crate::item_service_api::item_data::ItemData;

///AppState is a struct with current session as field
pub struct AppState {
    pub session: CurrentSession,
}

pub fn create_item(req: &HttpRequest<AppState>) -> Result<Item,CustomError> {
    let header= req.headers();

}

pub fn get_item(data: State<AppState>,item_id:Path<i32>) -> Result<Item,CustomError> {
    unimplemented!();
}

pub fn update_item(data: State<AppState>,item_id:Path<i32>,item_data: Json<ItemData>)
    -> Result<Item,CustomError> {
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
