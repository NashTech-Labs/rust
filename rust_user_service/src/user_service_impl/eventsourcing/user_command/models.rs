use crate::user_service_impl::models::user_registration::UserRegistration;

#[derive(Debug)]
pub enum UserCommand {
    CreateUser(UserRegistration),
}