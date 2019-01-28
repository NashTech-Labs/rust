use actix_web::{Path, Result};
use cdrs::query::QueryExecutor;
use constants::queries::DELETE_QUERY;
use env_set_up::connection::CurrentSession;
use repository::is_present::is_present;

/// this function is used to delete student data for particular id
pub fn delete_student(session: &CurrentSession, id: Path<i32>) -> Result<&'static str> {
    let roll_no = id.into_inner();
    if is_present(&session, roll_no) {
        Ok("student doesn't exist..")
    } else {
        session
            .query_with_values(DELETE_QUERY, query_values!(roll_no))
            .expect("delete");
        Ok("student deleted..")
    }
}
