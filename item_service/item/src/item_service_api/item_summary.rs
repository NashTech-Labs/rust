use crate::item_service_api::item_status::ItemStatus;

pub struct ItemSummary {
    pub id: i32,
    pub title: String,
    pub currency_id: String,
    pub reserve_price: f32,
    pub status: ItemStatus,

}