use crate::controller::error::CustomError;
use crate::models::item::Item;
use crate::models::item_data::{ItemData, PItemData};
use crate::models::item_status::{ItemStatus, PItemStatus};
use crate::models::location::Location;
use crate::models::p_item::PItem;

pub fn pitem_to_item(item: PItem) -> Item {
    let item_data: ItemData = pitemdata_to_itemdata(item.item_data);
    let item_status: ItemStatus = pitemstatus_to_itemstatus(item.status).unwrap();
    Item {
        id: item.id,
        creator: item.creator,
        item_data: item_data,
        price: item.price,
        status: item_status,
        auction_start: item.auction_start,
        auction_end: item.auction_end,
        auction_winner: item.auction_winner,
        location: Location {
            country: "".to_string(),
            state: "".to_string(),
            city: "".to_string(),
        },
        /*delivery_options: D*/
    }
}

pub fn pitemdata_to_itemdata(details: PItemData) -> ItemData {
    ItemData {
        title: details.title,
        description: details.description,
        currency_id: details.currency_id,
        increment: details.increment,
        reserve_price: details.reserve_price,
        auction_duration: details.auction_duration,
        category_id: details.category_id,
    }
}

pub fn itemdata_to_pitemdata(data: ItemData) -> PItemData {
    PItemData {
        title: data.title,
        description: data.description,
        currency_id: data.currency_id,
        increment: data.increment,
        reserve_price: data.reserve_price,
        auction_duration: data.auction_duration,
        category_id: data.category_id,
    }
}

pub fn pitemstatus_to_itemstatus(status: PItemStatus) -> Result<ItemStatus, CustomError> {
    match status {
        PItemStatus::CANCELLED => Ok(ItemStatus::CANCELLED),
        PItemStatus::CREATED => Ok(ItemStatus::CREATED),
        PItemStatus::COMPLETED => Ok(ItemStatus::COMPLETED),
        PItemStatus::AUCTION => Ok(ItemStatus::AUCTION),
        PItemStatus::NOT_CREATED => Err(CustomError::InternalError {
            field: "Item not created",
        }),
    }
}
