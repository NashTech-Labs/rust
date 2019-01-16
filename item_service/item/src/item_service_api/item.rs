use std::ptr::null;

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
    pub fn new(id: i32, creator: String,
               item_data: ItemData,
               price: f32,
               status: ItemStatus,
               auction_start: Option<String>,
               auction_end: Option<String>,
               auction_winner: Option<String>) -> Item {
        let item_status: Result<ItemStatus,CustomError> = match status {
            ItemStatus::CREATED => Ok(status),
            ItemStatus::AUCTION => Ok(status),
            ItemStatus::CANCELLED => Ok(status),
            ItemStatus::COMPLETED => Ok(status),
            _ => Err(CustomError::InvalidInput{field:"status doesn't set"})
        };
        Item {
            id,
            creator,
            item_data,
            price: if price != PRICE { price } else { PRICE },
            status: item_status,
            auction_start,
            auction_end,
            auction_winner,
            location: Location{

            },
            delivery_options: ()
        }
    }
}
