use actix_web::{Path, Result};
use cdrs::query::QueryExecutor;
use crud::is_present::is_present;
use env_set_up::connection::CurrentSession;

pub fn delete_struct(session: &CurrentSession, id: Path<i32>) -> Result<&'static str> {
    let roll_no = id.into_inner();
    if is_present(&session, roll_no)
        { Ok("student doesn't exist..") } else {
        let delete_struct_cql = "DELETE FROM student_ks.my_student_table WHERE roll_no = ? ";
        session
            .query_with_values(delete_struct_cql, query_values!(roll_no))
            .expect("delete");
        Ok("student deleted..")
    }
}
