use actix_web::{Json, Result};
use cdrs::query::*;
use constants::queries::UPDATE_QUERY;
use env_set_up::connection::CurrentSession;
use models::model::Student;
use repository::is_present::is_present;

/// this function is used to update student detail of particular id
pub fn update_student(
    session: &CurrentSession,
    student_details: &Json<Student>,
    path: i32,
) -> Result<&'static str> {
    if is_present(&session, path.to_owned()) {
       Ok("student doesn't exist")
    } else {
        let student: Student = Student {
            roll_no: path,
            marks: student_details.marks,
            name: student_details.name.clone(),
        };
        session
            .query_with_values(
                UPDATE_QUERY,
                query_values!(student.marks, student.name, student.roll_no),
            )
            .expect("update");
        Ok("student with this id updated")
    }
}
