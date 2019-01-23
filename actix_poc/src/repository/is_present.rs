use cdrs::query::QueryExecutor;
use constants::queries::SELECT_QUERY;
use env_set_up::connection::CurrentSession;

pub fn is_present(session: &CurrentSession, roll_no: i32) -> bool {
    session
        .query_with_values(SELECT_QUERY, query_values!(roll_no))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty()
}
