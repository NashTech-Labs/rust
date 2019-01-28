use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::Row;
use cdrs::{self, types::prelude::*};

/// this is schema of student detail
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct Student {
    pub roll_no: i32,
    pub name: String,
    pub marks: i32,
}