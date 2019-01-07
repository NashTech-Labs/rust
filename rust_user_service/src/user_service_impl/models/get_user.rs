use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};

/// UserMapper is used to map the details at retrieval time
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct UserMapper {
    pub user_id: String,
    pub user_state: String,
}