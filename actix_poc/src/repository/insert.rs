use actix_web::Result;
use cdrs::query::QueryExecutor;
use constants::queries::INSERT_QUERY;
use controller::error::CustomError;
use env_set_up::connection::CurrentSession;
use models::model::Student;
use repository::is_present::is_present;

pub fn insert_student(
    session: &CurrentSession,
    new_student: &Student,
) -> Result<&'static str, CustomError> {
    if is_present(&session, new_student.roll_no) {
        let student_json: String = serde_json::to_string(&new_student).unwrap();

        session
            .query_with_values(INSERT_QUERY, query_values!(student_json))
            .expect("insert error");
        Ok("welcome! student added")
    } else {
        Ok("student already exist")
    }
}
