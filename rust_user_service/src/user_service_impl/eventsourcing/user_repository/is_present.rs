use cdrs::query::QueryExecutor;
use crate::user_service_impl::env_setup::connection::CurrentSession;

pub fn is_present(session: &CurrentSession, e_mail: String) -> bool {
    let check_struct_cql = "Select * FROM user_ks.user_states WHERE email = ? ";
    session
        .query_with_values(check_struct_cql, query_values!(e_mail))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty()
}