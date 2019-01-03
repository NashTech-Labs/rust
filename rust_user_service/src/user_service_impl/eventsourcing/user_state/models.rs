use crate::user_service_impl::models::p_user::PUser;
use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserState {
    pub user: PUser,
    pub generation: u64,
}

impl AggregateState for UserState {
    fn generation(&self) -> u64 {
        self.generation
    }
}
