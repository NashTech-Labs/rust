use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::eventsourcing::user_repository::insertion::get_id_by_email;

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
                    id: get_id_by_email(&UserRegistration).unwrap().to_string(),
                    name: UserRegistration.name,
                    email: UserRegistration.email,
                    password: UserRegistration.password
                }),
            }
    }
}

