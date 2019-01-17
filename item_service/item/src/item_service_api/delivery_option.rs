use crate::item_service_api::location::Location;

pub enum DeliveryOption {
    Null,
    PickUp,
    Deliver {name: String, price: int, location: Location},
    ByNegotiation {location: Location},
}