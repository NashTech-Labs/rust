use actix_web::{Json, Result};
use cdrs::query::*;
use crud::isPresent::is_present;
use env_set_up::{connection::*, models::Student};

pub fn update_struct(session: &CurrentSession, new_student: Json<Student>, path: i32) -> Result<&'static str> {
    let is_exist = is_present(&session, path.to_owned());
    match is_exist {
        false => {
            let update_struct_cql = "UPDATE student_ks.my_student_table SET  marks=?,name=? WHERE roll_no = ? ";
            let stu: Student = Student {
                roll_no: path,
                marks: new_student.marks,
                name: new_student.name.clone(),
            };
            session
                .query_with_values(update_struct_cql, query_values!(stu.marks,stu.name,stu.roll_no))
                .expect("update");
            return Ok("student with this id updated");
        }
        true => return Ok("student doesn't exist")
    }
}
