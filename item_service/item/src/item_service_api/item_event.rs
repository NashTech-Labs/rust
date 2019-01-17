use crate::item_service_api::item::Item;
use crate::item_service_api::item_status::ItemStatus;

pub enum ItemEvent {
    ItemUpdated { item_id: i32, creator: String, title: String, description: String, item_status:
    ItemStatus, currency_id: String },
    AuctionStarted { item_id: i32, creator: String, reserve_price: f32, increment: f32, start_date:
    String, end_date: String },
    AuctionFinished { iyem_id: i32, item: Item},
    AuctionCancelled {item_id: i32}
}
