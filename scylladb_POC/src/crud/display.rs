use env_set_up::{connection::*,models::Student};
use actix_web::Path;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::*;
/*use mock_derive::mock;

#[mock]*/
pub fn select_struct(session: &CurrentSession, path: Path<i32>) -> Student{
    let select_struct_cql = "SELECT * FROM student_ks.my_student_table where roll_no = ?";
    let roll_no = path.into_inner();

    let rows = session.query_with_values(select_struct_cql, query_values!(roll_no))
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let mut my_row = Student {
        roll_no: 0,
        marks: 0,
        name: String::new(),
    };

    for row in rows {
        my_row = Student::try_from_row(row).expect("into Student")
    }

    my_row
}