use crate::item_service_api::item_data::ItemData;
use crate::item_service_api::item::Item;
use crate::item_service_impl::models::p_item::PItem;
use crate::item_service_api::item_status::ItemStatus;
use crate::item_service_api::location::Location;
use crate::item_service_impl::models::pitem_status::PItemStatus;
use crate::item_service_impl::controller::error::CustomError;
use crate::item_service_api::delivery_option::DeliveryOption;
use crate::item_service_impl::models::pitem_data::PItemData;

pub fn pitem_to_item(item: PItem) -> Item {
    let item_data: ItemData = pitemdata_to_itemdata(item.item_data);
    let item_status: ItemStatus = pitemstatus_to_itemstatus(item.status).unwrap();
    Item::new(item.id,
              item.creator,
              item_data,
              item.price,
              item_status,
              item.auction_start,
              item.auction_end,
              item.auction_winner,
    )
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
