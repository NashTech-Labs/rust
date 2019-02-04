use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::{self,types::prelude::*};
use cdrs::types::prelude::Row;
//use serde_json::to_string;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct Student {
    pub roll_no: i32,
    pub name: String,
    pub marks: i32,
}

/*
impl Student {
    pub fn typed_student(self) -> String{
       let stu = self;
        let j=to_string(&stu)?;
        j
    }
}*/
