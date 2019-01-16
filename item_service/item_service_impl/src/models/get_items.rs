use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};

/// UserMapper is used to map the details at retrieval time
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct ItemMapper {
    pub item_id: String,
    pub item_state: String,
}