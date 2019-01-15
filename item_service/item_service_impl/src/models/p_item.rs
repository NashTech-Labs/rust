extern  crate cdrs;
use std::time::Instant;
use cdrs::types::prelude::*;

use crate::models::item_data::{ItemData,PItemData};
use crate::models::item_status::PItemStatus;

#[derive(Clone, Debug, /*IntoCDRSValue, TryFromRow, */PartialEq, Serialize, Deserialize)]
pub struct PItem {
   //it should be private
    pub id: i32,
    pub creator: String,
    pub item_data: PItemData,
    pub price: f32,
    pub status: PItemStatus,
    //ends here
    pub auction_start: Option<Instant>,
    pub auction_end: Option<Instant>,
    pub auction_winner: Option<String>,
}

impl PItem {
    fn new(
        id: i32,
        creator: String,
        item_data: PItemData,
        price: f32,
        status: PItemStatus,
        auction_start: Option<Instant>,
        auction_end: Option<Instant>,
        auction_winner: Option<String>,
    ) -> PItem {
        PItem {
            id,
            creator,
            item_data,
            price,
            status,
            auction_start,
            auction_end,
            auction_winner,
        }
    }

    pub fn new_pitem(id: i32, creator: String, item_data: PItemData) -> PItem {
        PItem {
            id,
            creator,
            item_data,
            price: 0.0,
            status: PItemStatus::CREATED,
            auction_start: None,
            auction_end: None,
            auction_winner: None,
        }
    }
}
   /* pub fn start(start_time: Instant) -> PItem {
        assert_eq!(status, PItemStatus::CREATED);
        PItem {
            id,
            creator,
            item_data,
            price,
            status: PItemStatus::AUCTION,
            auction_start: Some(start_time),
            auction_end: Some(start_time.add(item_data.get_auction_duration())),
            auction_winner,
        }
    }

    pub fn end(auction_winner: Option<String>, price: f32) -> PItem {
        assert_eq!(status, PItemStatus::AUCTION);
        PItem {
            id,
            creator,
            item_data,
            price,
            status: PItemStatus::COMPLETED,
            auction_start,
            auction_end,
            auction_winner
        }
    }
    pub fn update_price(price:f32) -> PItem {
        assert_eq!(status, PItemStatus::AUCTION);
        PItem {
            id,
            creator,
            item_data,
            price,
            status,
            auction_start,
            auction_end,
            auction_winner
        }
    }
    pub fn cancel() -> PItem {
        assert_eq!(status, PItemStatus::AUCTION) || assert_eq!(status, PItemStatus::CREATED);
        PItem {
            id,
            creator,
            item_data,
            price,
            status:PItemStatus::CANCELLED,
            auction_start,
            auction_end,
            auction_winner
        }
    }
    pub fn with_details(item_data: PItemData) -> PItem {
        PItem {
            id,
            creator,
            item_data,
            price,
            status,
            auction_start,
            auction_end,
            auction_winner
        }
    }
    pub fn with_description(desc:String)-> PItem {
        PItem {
            id,
            creator,
            item_data:PItemData::with_description(desc),
            price,
            status,
            auction_start,
            auction_end,
            auction_winner
        }
    }
}
*/