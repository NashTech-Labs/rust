use actix_web::Result;
use cdrs::query::QueryExecutor;
use crud::is_present::is_present;
use env_set_up::connection::CurrentSession;
use models::model::Student;
use error::CustomError;

pub fn insert_struct(session: &CurrentSession, new_student: Student)
                     -> Result<&'static str, CustomError> {
        if is_present(&session, new_student.roll_no)
        {   let student_json: String = serde_json::to_string(&new_student).unwrap();
            let insert_struct_cql = "INSERT INTO student_ks.my_student_table Json ?";
            session.query_with_values(insert_struct_cql,
                                      query_values!( student_json))
                .expect("insert error");
            Ok("welcome! student added")
        }
        else {
        Ok("student already exist")
    }
}