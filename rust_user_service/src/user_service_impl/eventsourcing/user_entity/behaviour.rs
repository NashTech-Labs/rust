use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};
use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user_registration::UserRegistration;


impl Aggregate for PUser {
    type Event = UserEvent;
    type Command = UserCommand;
    type State = UserState;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let user_state: UserState = match evt {
            UserEvent::UserCreated(PUser) => UserState {
                user: PUser,
                generation: state.generation + 1,
            },
        };
        Ok(user_state)
    }

    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        Ok(vec![cmd.into()])
    }
}
