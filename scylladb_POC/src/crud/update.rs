use env_set_up::{connection::*,models::Student};
use actix_web::{Json,Path};
use cdrs::query::*;
/*use mock_derive::mock;

#[mock]*/
pub fn update_struct(session: &CurrentSession, new_student: Json<Student>, path: Path<i32>) {
    let update_struct_cql = "UPDATE student_ks.my_student_table SET  marks=?,name=? WHERE roll_no = ? If exists ";
    let stu: Student = Student {
        roll_no: path.into_inner(),
        marks: new_student.marks,
        name: new_student.name.clone(),
    };
    session
        .query_with_values(update_struct_cql, query_values!(stu.marks,stu.name,stu.roll_no))
        .expect("update");
}