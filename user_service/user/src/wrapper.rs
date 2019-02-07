/// Outcomes is used to wrap Vec<User>
#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct Outcomes<T> {
    pub outcomes: Vec<T>,
}

/// wrap_vec is used to map Vec<User> into Outcomes
pub fn wrap_vec<T>(v: Vec<T>) -> Outcomes<T> {
    Outcomes { outcomes: v }
}
