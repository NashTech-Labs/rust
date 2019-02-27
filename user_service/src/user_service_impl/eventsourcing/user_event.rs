use crate::user_service_impl::eventsourcing::user_entity::PUser;

/// UserEvent is a struct which holds all events related to User
#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("")]
pub enum UserEvent {
    /// This variant will be trigger when createUser type command occurs
    UserCreated(PUser),
}
