use actix_web::{Json, Path, Result};
use scylladb_poc::env_set_up::{connection::*,keyspace::*,table::*,models::*};
use scylladb_poc::crud::{insert::*,delete::*,update::*,display::*};

pub fn insert(student: Json<Student>) -> Result<String> {
    let session = &connect();
    create_keyspace(&session);
    create_table(&session);
    insert_struct(&session, student);
    Ok(format!("Welcome ! student added "))
}

pub fn show(path: Path<i32>) -> Result<Json<Student>> {
    let student = select_struct(&connect(), path);
    Ok(Json(Student {
        roll_no: student.roll_no,
        marks: student.marks,
        name: student.name.clone(),
    }))
}

pub fn delete(path: Path<i32>) -> Result<String> {
    delete_struct(&connect(), path);
    Ok(format!(" student deleted "))
}

pub fn update(student: Json<Student>, path: Path<i32>) -> Result<String> {
    update_struct(&connect(), student, path);
    Ok(format!(" student updated "))
}

