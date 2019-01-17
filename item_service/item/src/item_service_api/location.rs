#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
}