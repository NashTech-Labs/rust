use actix_web::{Json, Path, Result};
use scylladb_poc::crud::{delete::*, display::*, insert::*, update::*};
use scylladb_poc::env_set_up::{connection::*, keyspace::*, models::*, table::*};
use scylladb_poc::error::*;

pub fn insert(student: Json<Student>) -> Result<&'static str, CustomError> {
    let session = &connect();
    let stu: Student = student.into_inner();
    match stu.marks <= 100 {
        true => {
            match stu.name == "" {
                true => Err(CustomError::InvalidInput {
                    field: "name should not be null"
                }),
                false => {
                    create_keyspace(&session);
                    create_table(&session);
                    insert_struct(&session, stu)
                }
            }
        }
        false => {
            Err(CustomError::InvalidInput {
                field: "marks should not be greater than 100"
            })
        }
    }
}

pub fn show(path: Path<i32>) -> Result<Json<Option<Student>>, CustomError> {
    let student = select_struct(&connect(), path);
    let stu_clone = student.clone();
    match stu_clone {
        Some(_T) => {
            let stu = student.unwrap();
            Ok(Json(Some(Student {
                roll_no: stu.roll_no,
                marks: stu.marks,
                name: stu.name.clone(),
            })))
        }
        None => Err(CustomError::InvalidInput { field: "student with this id doesn't exist" })
    }
}

pub fn delete(path: Path<i32>) -> Result<&'static str> {
    delete_struct(&connect(), path)
}

pub fn update(student: Json<Student>, path: Path<i32>) -> Result<&'static str> {
    update_struct(&connect(), student, path.into_inner())
}

