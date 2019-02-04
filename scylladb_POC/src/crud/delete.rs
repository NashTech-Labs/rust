use env_set_up::connection::*;
use actix_web::Path;
use cdrs::query::QueryExecutor;
/*use mock_derive::mock;

#[mock]*/
pub fn delete_struct(session: &CurrentSession, id: Path<i32>) {
    let delete_struct_cql = "DELETE FROM student_ks.my_student_table WHERE roll_no = ? ";
    let roll_no = id.into_inner();
    session
        .query_with_values(delete_struct_cql, query_values!(roll_no))
        .expect("delete");
}
