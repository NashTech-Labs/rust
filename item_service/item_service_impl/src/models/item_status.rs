use controller::error::CustomError;

pub enum ItemStatus {
    CREATED,
    AUCTION,
    COMPLETED,
    CANCELLED,
}

pub enum PItemStatus {
    NOT_CREATED,
    CREATED,
    AUCTION,
    COMPLETED,
    CANCELLED
}


impl PItemStatus {

    fn toitemstatus(&self) -> Result<ItemStatus,CustomError> {
        match  &self {
            PItemStatus::NOT_CREATED => Err(CustomError::InternalError {field: "Publically exposed Item cant't be created"}),
            PItemStatus::CREATED => Ok(ItemStatus::CREATED),
            PItemStatus::CANCELLED => Ok(ItemStatus::CANCELLED),
            PItemStatus::COMPLETED => Ok(ItemStatus::COMPLETED),
            PItemStatus::AUCTION => Ok(ItemStatus::AUCTION),
        }
    }
}
