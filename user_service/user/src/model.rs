use validator::{Validate, ValidationError};

/// User is used to map the details at ClientResponse time
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

/// UserLogin is used to map the details at ClientRequest time for login end-point
#[derive(Debug, Deserialize, Serialize, Validate, Clone, PartialEq)]
pub struct UserLogin {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}

/// UserRegistration is used to map the details at ClientRequest time at create_user end-point
#[derive(Debug, Deserialize, Serialize, Validate, Clone, PartialEq)]
pub struct UserRegistration {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}
