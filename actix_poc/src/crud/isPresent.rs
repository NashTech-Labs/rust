use cdrs::query::QueryExecutor;
use env_set_up::connection::*;

pub fn is_present(session: &CurrentSession, roll_no: i32) -> bool {
    let check_struct_cql = "Select * FROM student_ks.my_student_table WHERE roll_no = ? ";
    let row = session
        .query_with_values(check_struct_cql, query_values!(roll_no))
        .expect("isPresent error")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows")
        .is_empty();
    row
}