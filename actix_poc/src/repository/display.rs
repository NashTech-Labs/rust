use actix_web::Path;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::*;
use constants::constant::ZERO;
use constants::queries::SELECT_QUERY;
use env_set_up::connection::CurrentSession;
use models::model::Student;
use std::cell::RefCell;

pub fn select_student(session: &CurrentSession, student: Path<i32>) -> Option<Student> {
    let roll_no: i32 = student.into_inner();

    let student_rows: Vec<Row> = session
        .query_with_values(SELECT_QUERY, query_values!(roll_no))
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    if student_rows.is_empty() {
        None
    } else {
        let get_student_list: RefCell<Vec<Student>> = RefCell::new(vec![]);
        for row in student_rows {
            get_student_list
                .borrow_mut()
                .push(Student::try_from_row(row).expect("into get user"));
        }
        let result: Vec<Student> = get_student_list.borrow().to_vec();
        let student: Student = result[ZERO].to_owned();
        Some(student)
    }
}
