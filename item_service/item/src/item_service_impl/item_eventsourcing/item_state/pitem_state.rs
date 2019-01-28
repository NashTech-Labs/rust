use std::time::Instant;
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};
use eventsourcing::AggregateState;
use crate::item_service_impl::models::p_item::PItem;
use crate::item_service_impl::models::pitem_data::PItemData;
use crate::item_service_impl::models::pitem_status::PItemStatus;

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
        PItemState::update(|i| i::update_price(price))
    }

    pub fn update_details(details: PItemData) -> PItemState {
        PItemState::update(|i| i.with_details(details))
    }

    pub fn cancel(&self) -> PItemState {
        PItemState::update(&self,&self.item.map(|i|i.cancel()))
    }

    pub fn get_status(&self) -> PItemStatus {
        if &self.item.status != PItemStatus::NOT_CREATED
            { item.status } else { PItemStatus::NOT_CREATED }
    }
    fn update(&self, pitem: PItem)-> PItemState {
        assert_eq!(&self.item, Some(&self.item));
        PItemState::new(self.item).item.map(pitem)
    }/*
     fn update<F>(&self,f: F) -> PItemState where F: Fn(PItem) -> PItem {
        assert_eq!(&self.item, Some(&self.item));
        PItemState::new(self.item).item.map(f)
    }*/
}

impl AggregateState for PItemState {
    fn generation(&self) -> u64 {
        self.generation
    }
}