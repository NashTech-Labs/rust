use env_set_up::{connection::*,models::Student};
use actix_web::Json;
use cdrs::query::QueryExecutor;
use serde_json::to_string;

pub fn insert_struct(session: &CurrentSession, new_student: Json<Student>) {
   let stu = Student{
        roll_no: new_student.roll_no,
        marks: new_student.marks,
        name: new_student.name.clone(),

    };
    let j= to_string(&stu).unwrap();

    let insert_struct_cql = "INSERT INTO student_ks.my_student_table Json ?";
    session//.query(insert_struct_cql)
        .query_with_values(insert_struct_cql, query_values!( j))
        .expect("insert here ");
}
