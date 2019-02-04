use actix_web::{HttpResponse, Json, Result};
use cdrs::query::QueryExecutor;
use crud::isPresent::*;
use env_set_up::{connection::*, models::*};
use serde_json::to_string;

pub fn insert_struct(session: &CurrentSession, new_student: Json<Student>) -> Result<&'static str> {
    let stu = Student {
        roll_no: new_student.roll_no,
        marks: new_student.marks,
        name: new_student.name.clone(),
    };
    let is_exist = is_present(&session, stu.roll_no);
    match is_exist {
        true => {
            let j = serde_json::to_string(&stu).unwrap();

            let insert_struct_cql = "INSERT INTO student_ks.my_student_table Json ?";
           session.query_with_values(insert_struct_cql, query_values!( j))
                .expect("insert here ");
            return Ok("welcome! student added");
        }
        false => return Ok("student already exist")
    }
}
