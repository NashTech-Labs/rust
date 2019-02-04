use actix_web::{error, HttpResponse, Json, Path, Result};
use scylladb_poc::crud::{delete::*, display::*, insert::*, update::*};
use scylladb_poc::env_set_up::{connection::*, keyspace::*, models::*, table::*};

pub fn insert(student: Json<Student>) -> Result<&'static str> {
    let session = &connect();
    create_keyspace(&session);
    create_table(&session);
    insert_struct(&session, student)
}

pub fn show(path: Path<i32>) -> Result<Json<Student>> {
    let student = select_struct(&connect(), path);
    Ok(Json(Student {
        roll_no: student.roll_no,
        marks: student.marks,
        name: student.name.clone(),
    }))
}

pub fn delete(path: Path<i32>) -> Result<&'static str> {
    delete_struct(&connect(), path)
}

pub fn update(student: Json<Student>, path: Path<i32>) -> Result<&'static str> {
    update_struct(&connect(), student, path.into_inner())
}

