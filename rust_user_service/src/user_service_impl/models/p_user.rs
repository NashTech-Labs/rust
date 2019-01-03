use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self, types::prelude::*};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct PUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
