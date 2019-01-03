use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user_registration::UserRegistration;
use uuid::parser::ParseError;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("")]
pub enum UserEvent {
    UserCreated(PUser),
}
/*

impl From<UserCommand> for UserEvent {
    fn from(source: UserCommand) -> Self {
        match source {
            UserCommand::CreateUser(new_user) =>
                UserEvent::UserCreated(PUser{
                    id: get_id_by_email(&new_user).unwrap().to_string(),
                    name: new_user.name,
                    email: new_user.email,
                    password: new_user.password
                }),
            }
    }
}*/
