#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    country: String,
    state: String,
    city: String,
}