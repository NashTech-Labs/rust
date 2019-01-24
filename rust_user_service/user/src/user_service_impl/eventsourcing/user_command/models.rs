use crate::user_service_api::models::user_registration::UserRegistration;

/// UserCommand is a struct which holds following commands for user as field
#[derive(Debug)]
pub enum UserCommand {
    /// This variant is to create a new user
    CreateUser(UserRegistration),
}