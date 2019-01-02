use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::models::p_user::PUser;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("")]
pub enum UserEvent {
    UserCreated(PUser),
}

impl From<UserCommand> for UserEvent {
    fn from(source: UserCommand) -> Self {
        match source {
            UserCommand::CreateUser(UserRegistration) =>
                UserEvent::UserCreated(PUser{
                    id: get_id_by_email(&UserRegistration).to_string(),
                    name: UserRegistration.name,
                    email: UserRegistration.email,
                    password: UserRegistration.password
                }),
            }
    }
}

/// this method is used to retrieve the id from email
fn get_id_by_email(user_reg:&UserRegistration)-> Uuid {
    let bytes= user_reg.email.to_lowercase().bytes();
    let user_id = Uuid::from_bytes(bytes);
    user_id
}