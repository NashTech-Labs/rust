use crate::model::User;

/// Outcomes is used to wrap Vec<User>
#[derive(Serialize, Debug, PartialEq,Deserialize)]
pub struct Outcomes {
    pub outcomes: Vec<User>,
}

/// wrap_vec is used to map Vec<User> into Outcomes
pub fn wrap_vec(v: Vec<User>) -> Outcomes {
    Outcomes {
        outcomes: v
    }
}