use crate::item_service_api::location::Location;

#[derive(Clone, Debug,PartialEq, Serialize, Deserialize)]
pub enum DeliveryOption {
    Null,
    PickUp,
    Deliver {name: String, price: i32, location: Location},
    ByNegotiation {location: Location},
}