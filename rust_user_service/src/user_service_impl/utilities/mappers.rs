use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user::User;

<<<<<<< HEAD
=======
/// map_user is used to map PUser into User
>>>>>>> 94334322fddb5eacaafb99cc2707c5f28874c647
pub fn map_user(user: PUser) -> User {
    User {
        id: user.id,
        name: user.name,
        email: user.email,
    }
}
