use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user::User;

pub fn map_user(user: PUser) -> User {
    User {
        id: user.id,
        name: user.name,
        email: user.email,
    }
}
