use actix_web::{Json, Path, Result};
use crud::{delete::delete_struct, display::select_struct, insert::insert_struct, update::update_struct};
use env_set_up::connection::connect;
use models::model::Student;
use error::CustomError;

pub fn insert(student: Json<Student>) -> Result<&'static str, CustomError> {
    let session = &connect();
    let stu: Student = student.into_inner();
    if stu.marks <= 100 {
        if stu.name == ""
            { Err(CustomError::InvalidInput {
                field: "name should not be null"
            })
            } else {
            insert_struct(&session, stu)
        }
    } else {
        Err(CustomError::InvalidInput {
            field: "marks should not be greater than 100"
        })
    }
}

pub fn show(path: Path<i32>) -> Result<Json<Option<Student>>, CustomError> {
    let student = select_struct(&connect(), path);
    let stu_clone = student.clone();
    match stu_clone {
        Some(_t) => {
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

