use crate::item_service_api::item_data::ItemData;
use crate::item_service_api::item_status::ItemStatus;
use crate::item_service_api::location::Location;
use crate::item_service_impl::controller::error::CustomError;
use crate::item_service_impl::constants::constants::PRICE;
use std::ptr::null;
use crate::item_service_api::delivery_option::DeliveryOption;

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
    location: Location,
    delivery_options: DeliveryOptions,
}

impl Item {
    fn new(id: i32, creator: String,
               item_data: ItemData,
               price: f32,
               status: ItemStatus,
               auction_start: Option<String>,
               auction_end: Option<String>,
               auction_winner: Option<String>) -> Item {

        Item {
            id,
            creator,
            item_data,
            price: if price != PRICE { price } else { PRICE },
            status: if status != ItemStatus::NULL { status} else {ItemStatus::NULL},
            auction_start,
            auction_end,
            auction_winner,
            location: Location{
                country: None,
                state: None,
                city: None
            },
            delivery_options: DeliveryOption::Null,
        }
    }

    pub fn new_item(&self,id: i32, creator: String,
           item_data: ItemData,
           price: f32,
           status: ItemStatus,
           auction_start: Option<String>,
           auction_end: Option<String>,
           auction_winner: Option<String>) {

            self.id: id;
            self.creator: creator,
        selfitem_data,
        selfprice,
        selfstatus,
        selfauction_start,
        selfauction_end,
        selfauction_winner,
            }
    }
}
