use env_set_up::connection::*;
use cdrs::query::QueryExecutor;

pub fn create_keyspace(session: &CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS student_ks WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    let a = session
        .query(create_ks)
        ;
}