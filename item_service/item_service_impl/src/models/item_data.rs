use std::time::Duration;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemData {
    pub title: String,
    pub description: String,
    pub currency_id: String,
    pub increment: f32,
    pub reserve_price: f32,
    pub auction_duration: Duration,
    pub category_id: Option<i32>,
}