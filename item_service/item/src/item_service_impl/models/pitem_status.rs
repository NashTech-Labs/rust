use crate::item_service_api::item_status::ItemStatus;
use crate::item_service_impl::controller::error::CustomError;

#[derive(Debug,Clone,PartialEq,Serialize, Deserialize)]
pub enum PItemStatus {
    NOT_CREATED,
    CREATED,
    AUCTION,
    COMPLETED,
    CANCELLED
}

impl PItemStatus {
    pub fn toitemstatus(&self) -> Result<ItemStatus,CustomError> {
        match  &self {
            PItemStatus::NOT_CREATED => Err(CustomError::InternalError {field:
            "Publically exposed Item cant't be created"}),
            PItemStatus::CREATED => Ok(ItemStatus::CREATED),
            PItemStatus::CANCELLED => Ok(ItemStatus::CANCELLED),
            PItemStatus::COMPLETED => Ok(ItemStatus::COMPLETED),
            PItemStatus::AUCTION => Ok(ItemStatus::AUCTION),
        }
    }
}
