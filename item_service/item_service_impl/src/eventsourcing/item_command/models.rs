use crate::models::p_item::PItem;
use crate::models::item_data::ItemData;

#[derive(Debug,Clone.Serialize,Deserialize)]
pub enum PItemCommand {
    CreateItem(PItem),
    UpdateItem { commander: String, item_data: ItemData },
    StartAuction {user_id: String},
    UpdatePrice{price: f32},
    FinishAuction{winner: Option<String>,price: f32}
}