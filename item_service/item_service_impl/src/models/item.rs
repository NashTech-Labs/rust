use crate::models::item_data::ItemData;
use crate::models::location::Location;
use std::time::Instant;
use crate::models::item_status::ItemStatus;

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
    pub fn new(id: i32,creator: String) -> Item {
        Item {
            id: 0,
            creator: String::new(),
            item_data: ItemData{
                title: String::new(),
                description: String::new(),
                currency_id: String::new(),
                increment: 0.0,
                reserve_price: 0.0,
                auction_duration: ,
                category_id: None
            },
            price: 0.0,
            status: (),
            auction_start: None,
            auction_end: None,
            auction_winner: None,
            location: Location{
                country: String::new(),
                state: String::new(),
                city: String::new()
            },
            delivery_options: ()
        }
    }
}
