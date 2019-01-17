#[derive(Debug,Clone,PartialEq,Serialize, Deserialize)]
pub enum ItemStatus {
    NULL,
    CREATED,
    AUCTION,
    COMPLETED,
    CANCELLED,
}
