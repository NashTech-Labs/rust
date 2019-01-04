use crate::user_service_impl::controller::handler::get_id_by_email;
use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::p_user::PUser;
use eventsourcing::{Result,Aggregate};

impl Aggregate for PUser {
    type Event = UserEvent;
    type Command = UserCommand;
    type State = UserState;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let user_state: UserState = match evt {
            UserEvent::UserCreated(puser) => UserState {
                user: puser,
                generation: state.generation + 1,
            },
        };
        Ok(user_state)
    }

    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        let user_event: UserEvent = match cmd {
            UserCommand::CreateUser(new_user) => UserEvent::UserCreated(PUser {
                id: get_id_by_email(&new_user).to_string(),
                name: new_user.name,
                email: new_user.email,
                password: new_user.password,
            }),
        };
        Ok(vec![user_event])
    }
}
