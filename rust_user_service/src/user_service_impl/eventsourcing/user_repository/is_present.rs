use cdrs::query::QueryExecutor;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::constants::queries::IS_PRESENT_QUERY;

pub fn is_present(session: &CurrentSession, id: String) -> bool {
        session
        .query_with_values(IS_PRESENT_QUERY, query_values!(id))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty()
}