use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::p_user::PUser;
use eventsourcing::{Result,Aggregate};
use crate::user_service_api::user_service::handler::get_id_by_email;

impl Aggregate for PUser {
    type Event = UserEvent;
    type Command = UserCommand;
    type State = UserState;

    /// apply_event takes events and state as input parameter and returns result of states
    #[cfg_attr(tarpaulin,skip)]
    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let user_state: UserState = match evt {
            UserEvent::UserCreated(puser) => UserState {
                user: puser,
                generation: state.generation + 1,
            },
        };
        Ok(user_state)
    }

    /// handle_command takes command and state as input parameter and
    /// returns result of vector of events
    #[cfg_attr(tarpaulin,skip)]
    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        let user_event: UserEvent = match cmd {
            UserCommand::CreateUser(new_user) => UserEvent::UserCreated(PUser {
                id: get_id_by_email(new_user.email.as_str()).to_string(),
                name: new_user.name,
                email: new_user.email,
                password: new_user.password,
            }),
        };
        Ok(vec![user_event])
    }
}
