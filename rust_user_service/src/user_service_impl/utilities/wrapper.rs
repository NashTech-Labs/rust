use crate::user_service_impl::models::user::User;

#[derive(Serialize, Debug, Deserialize)]
pub struct Outcomes {
    pub outcomes: Vec<User>,
}

pub fn wrap_vec(v: Vec<User>) -> Outcomes {
    Outcomes {
        outcomes: v
    }
}