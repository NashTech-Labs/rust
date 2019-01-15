#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub country: String,
    pub state: String,
    pub city: String,
}