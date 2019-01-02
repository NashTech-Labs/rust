#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
}