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

impl ItemData {
    pub fn new(title: String,
               description: String,
               currency_id: String,
               increment: f32,
               reserve_price: f32,
               auction_duration: Duration,
               category_id: Option<i32>) -> ItemData {
        ItemData {
            title,
            description,
            currency_id,
            increment,
            reserve_price,
            auction_duration,
            category_id
        }
    }
}