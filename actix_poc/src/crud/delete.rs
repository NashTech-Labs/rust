use actix_web::{Path, Result};
use cdrs::query::QueryExecutor;
use crud::isPresent::*;
use env_set_up::connection::*;

pub fn delete_struct(session: &CurrentSession, id: Path<i32>) -> Result<&'static str> {
    let roll_no = id.into_inner();
    let is_exist = is_present(&session, roll_no.clone());
    match is_exist {
        true => return Ok("student doesn't exist.."),
        false => {
            let delete_struct_cql = "DELETE FROM student_ks.my_student_table WHERE roll_no = ? ";
            session
                .query_with_values(delete_struct_cql, query_values!(roll_no))
                .expect("delete");
            return Ok("student deleted..");
        }
    }
}
