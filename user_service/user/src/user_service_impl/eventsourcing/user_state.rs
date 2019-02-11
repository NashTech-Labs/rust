use crate::user_service_impl::eventsourcing::user_entity::PUser;
use eventsourcing::prelude::*;

///UserState is a struct which holds persistent user and its generation
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserState {
    pub user: PUser,
    pub generation: u64,
}

impl AggregateState for UserState {
    #[cfg_attr(tarpaulin, skip)]
    fn generation(&self) -> u64 {
        self.generation
    }
}
