use std::time::Instant;
use crate::models::item_status::ItemStatus;
use crate::models::p_item::PItem;

pub enum ItemEvent {
    ItemUpdated { item_id: i32, creator: String, title: String, description: String, item_status: ItemStatus, currency_id: String },
    AuctionStarted { item_id: i32, creator: String, reserve_price: f32, increment: f32, start_date: Instant, end_date: Instant },
}

pub enum PItemEvent {
    ItemCreated(PItem),
    ItemUpdated { item_id: i32, creator: String, item_details: PItemData, item_status: PItemStatus },
    AuctionStarted { item_id: i32, start_time: Instant },
    PriceUpdated { item_id: i32, price: f32 },
    AuctionFinished { item_id: i32, winner: Option<String>, price: f32 },
}