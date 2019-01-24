/// UserRegistration is used to map the details at ClientRequest time at create_user end-point
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
}