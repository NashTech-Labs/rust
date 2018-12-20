use actix_web::{Result};
use cdrs::query::QueryExecutor;
use crud::isPresent::*;
use env_set_up::{connection::*, models::*};
use error::CustomError;

pub fn insert_struct(session: &CurrentSession, new_student: Student)
                     -> Result<&'static str, CustomError> {
    let is_exist = is_present(&session, new_student.roll_no);
    match is_exist {
        true => {
            let student_json: String = serde_json::to_string(&new_student).unwrap();

            let insert_struct_cql = "INSERT INTO student_ks.my_student_table Json ?";
            session.query_with_values(insert_struct_cql,
                                      query_values!( student_json))
                .expect("insert error");
            return Ok("welcome! student added");
        }
        false => return Ok("student already exist")
    }
}
