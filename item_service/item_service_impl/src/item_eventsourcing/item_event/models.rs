use crate::models::item_status::ItemStatus;
use crate::models::p_item::PItem;
use crate::models::item_data::PItemData;
use crate::models::item_status::PItemStatus;

pub enum ItemEvent {
    ItemUpdated { item_id: i32, creator: String, title: String, description: String, item_status: ItemStatus, currency_id: String },
    AuctionStarted { item_id: i32, creator: String, reserve_price: f32, increment: f32, start_date: String, end_date: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("")]
pub enum PItemEvent {
    ItemCreated{item: PItem},
    ItemUpdated { item_id: i32, creator: String, item_details: PItemData, item_status: PItemStatus },
    AuctionStarted { item_id: i32, start_time: String },
    PriceUpdated { item_id: i32, price: f32 },
    AuctionFinished { item_id: i32, winner: Option<String>, price: f32 },
    ItemRetrieved,
}

/*
pub fn get_itemid(item: PItem) -> i32 {
    item.item_id
}*/