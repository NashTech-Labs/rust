/// User is used to map the details at ClientResponse time
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

