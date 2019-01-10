use crate::models::item::Item;
use crate::models::p_item::PItem;
use crate::models::item_data::ItemData;

pub fn pitem_to_item (item:PItem) -> Item {
            Item {
                id: item.id,
                creator:item.creator,
                item_data: item.item_data,
                price: item.price,
                status: item.status,
                auction_start: item.auction_start,
                auction_end: item.auction_end,
                auction_winner: item.auction_winner,
            }
}