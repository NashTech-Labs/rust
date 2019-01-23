pub static KEYSPACE: &str = "CREATE KEYSPACE IF NOT EXISTS student_ks WITH REPLICATION = { \
                             'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
pub static TABLE: &str =
    "CREATE TABLE IF NOT EXISTS student_ks.my_student_table (roll_no int PRIMARY KEY , \
     name text, marks int);";

pub static UPDATE_QUERY: &str =
    "UPDATE student_ks.my_student_table SET  marks=?,name=? WHERE roll_no = ? ";

pub static SELECT_QUERY: &str = "Select * FROM student_ks.my_student_table WHERE roll_no = ? ";

pub static INSERT_QUERY: &str = "INSERT INTO student_ks.my_student_table Json ?";

pub static DELETE_QUERY: &str = "DELETE FROM student_ks.my_student_table WHERE roll_no = ? ";
