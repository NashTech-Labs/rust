use std::ptr::null;

use crate::constants::constants::ZERO;
use crate::models::item_data::ItemData;
use crate::models::item_status::ItemStatus;
use crate::models::location::Location;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub creator: String,
    pub item_data: ItemData,
    pub price: f32,
    pub status: ItemStatus,
    pub auction_start: Option<String>,
    pub auction_end: Option<String>,
    pub auction_winner: Option<String>,
    pub location: Location,
    /*delivery_options: DeliveryOptions,*/
}

impl Item {
    pub fn new(id: i32, creator: String,
               item_data: ItemData,
               price: f32,
               status: ItemStatus,
               auction_start: Option<String>,
               auction_end: Option<String>,
               auction_winner: Option<String>) -> Item {
        let item_status: ItemStatus = match status {
            ItemStatus::CREATED => status,
            ItemStatus::AUCTION => status,
            ItemStatus::CANCELLED => status,
            ItemStatus::COMPLETED => status,
        };
        Item {
            id,
            creator,
            item_data,
            price: if price != ZERO { price } else { ZERO },
            status: item_status,
            auction_start,
            auction_end,
            auction_winner,
            location: Location{
                country: "".to_string(),
                state: "".to_string(),
                city: "".to_string()
            },
            //delivery_options: None,
        }
    }
}
