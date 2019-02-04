use std::time::Instant;
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};
use eventsourcing::AggregateState;
use crate::item_service_impl::models::p_item::PItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PItemState {
    pub item: Option<PItem>,
    pub generation: u64,
}

impl PItemState {
    pub fn new(pitem: Option<PItem>) -> PItemState {
        PItemState {
            item: pitem,
            generation: 0,
        }
    }

    pub fn empty() -> PItemState {
        PItemState { item: None, generation: 0 }
    }

    pub fn create(item: PItem) -> PItemState {
        PItemState { item: Some(item), generation: 0 }
    }

    pub fn start(start_time: Instant) -> PItemState {
        PItemState::update(|i| i.start(start_time))
    }

    pub fn end(winner: Option<String>, price: f32) -> PItemState {
        PItemState::update(|i| i.end(winner, price))
    }

    pub fn update_price(price: f32) -> PItemState {
        PItemState::update(|i| i.update_price(price))
    }

    pub fn update_details(details: PItemData) -> PItemState {
        PItemState::update(|i| i.with_details(details))
    }

    pub fn cancel() -> PItemState {
        PItemState::update(|i| i.cancel())
    }

    pub fn get_status(item: PItem) -> PItemStatus {
        if (item.status != PItemStatus::NOT_CREATED)
            { item.status } else { PItemStatus::NOT_CREATED }
    }

    fn update<F>(&self,f:F) -> PItem where F:Fn(PItem) -> PItem {
    /*unimplemented!()*/

    }
}

impl AggregateState for PItemState {
    fn generation(&self) -> u64 {
        self.generation
    }
}