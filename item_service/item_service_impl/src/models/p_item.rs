use crate::models::item_data::ItemData;
use std::time::Instant;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]
pub struct PItem {
    id: i32,
    creator: String,
    item_data: ItemData,
    price: f32,
    status: PItemStatus,
    pub auction_start: Option<Instant>,
    pub auction_end: Option<Instant>,
    pub auction_winner: Option<String>,
}

impl PItem {}