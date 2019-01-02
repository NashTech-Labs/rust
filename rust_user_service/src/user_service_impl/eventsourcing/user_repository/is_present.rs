use cdrs::query::QueryExecutor;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use uuid::Uuid;

pub fn is_present(session: &CurrentSession, id: Uuid) -> bool {
    let check_struct_cql = "Select * FROM user_ks.user_states WHERE id = ? ";
    session
        .query_with_values(check_struct_cql, query_values!(id))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty()
}