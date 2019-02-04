use cdrs::query::QueryExecutor;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::constants::queries::SELECT_QUERY;

/// is_present is used to check whether a particular user's state is exists in database or not
pub fn is_present(session: &CurrentSession, id: String) -> bool {
        session
        .query_with_values(SELECT_QUERY, query_values!(id))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty()
}