#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}
