pub const KEYSPACE: &str = "CREATE KEYSPACE IF NOT EXISTS student_ks WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
pub const TABLE: &str = "CREATE TABLE IF NOT EXISTS student_ks.my_student_table (roll_no int PRIMARY KEY , \
     name text, marks int);";