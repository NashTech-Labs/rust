use crate::user_service_impl::eventsourcing::user_command::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::UserState;
use crate::user_service_impl::handler::get_id_by_email;
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use eventsourcing::{Aggregate, Result};

/// initial_state is used to set the initial value of UserState
pub fn initial_state() -> UserState {
    UserState {
        user: PUser {
            id: "".to_string(),
            name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        },
        generation: 0,
    }
}

/// PUser is used to map the details at storing time
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct PUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Aggregate for PUser {
    type Event = UserEvent;
    type Command = UserCommand;
    type State = UserState;

    /// apply_event takes events and state as input parameter and returns result of states
    #[cfg_attr(tarpaulin, skip)]
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
    #[cfg_attr(tarpaulin, skip)]
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
