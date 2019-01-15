use crate::models::item::Item;
use crate::models::p_item::PItem;
use crate::models::item_data::{ItemData,PItemData};
use crate::models::location::Location;

pub fn pitem_to_item (item:PItem) -> Item {

    let item_data:ItemData = pitemdata_to_itemdata(item.item_data);
            Item {
                id: item.id,
                creator:item.creator,
                item_data: item_data,
                price: item.price,
                status: item.status,
                auction_start: item.auction_start,
                auction_end: item.auction_end,
                auction_winner: item.auction_winner,
                location: Location{
                    country: "".to_string(),
                    state: "".to_string(),
                    city: "".to_string()
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
        category_id: details.category_id
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
        category_id: data.category_id
    }
}

