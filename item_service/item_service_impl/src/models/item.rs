use crate::models::item_data::ItemData;
use crate::models::location::Location;
use std::time::Instant;
use crate::models::item_status::ItemStatus;
use crate::constants::constants::ZERO;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub creator: String,
    pub item_data: ItemData,
    pub price: f32,
    pub status: ItemStatus,
    pub auction_start: Option<Instant>,
    pub auction_end: Option<Instant>,
    pub auction_winner: Option<String>,
    location: Location,
    delivery_options: DeliveryOptions,
}

impl Item {
    pub fn new(id: i32,creator: String,
               item_data: ItemData,
                price: f32,
                status: ItemStatus,
                auction_start: Option<Instant>,
                auction_end: Option<Instant>,
                auction_winner: Option<String>) -> Item {
        Item {
            id,
            creator,
            item_data,
            price: price.bitor_assign(ZERO),
            status: if status {status} else {Nil},
            auction_start ,
            auction_end,
            auction_winner,
            location: None,
            delivery_options: None,
        }
    }
}
