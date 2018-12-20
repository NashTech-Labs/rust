use env_set_up::connection::*;
use cdrs::query::QueryExecutor;

pub fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE IF NOT EXISTS student_ks.my_student_table (roll_no int PRIMARY KEY , \
     name text, marks int);";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}