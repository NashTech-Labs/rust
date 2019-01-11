/// UserLogin is used to map the details at ClientRequest time for login end-point
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}