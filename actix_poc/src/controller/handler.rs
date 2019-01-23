use actix_web::State;
use actix_web::{Json, Path, Result};
use constants::constant::{MAXIMUM_MARKS, ZERO};
use controller::error::CustomError;
use env_set_up::connection::CurrentSession;
use models::model::Student;
use repository::{
    delete::delete_student, display::select_student, insert::insert_student, update::update_student,
};

pub struct AppState {
    pub session: CurrentSession,
}

pub fn insert(data: State<AppState>, student: Json<Student>) -> Result<&'static str, CustomError> {
    let student: Student = student.into_inner();
    if student.marks <= MAXIMUM_MARKS {
        if student.name.len() == ZERO {
            Err(CustomError::InvalidInput {
                field: "name should not be empty",
            })
        } else {
            insert_student(&data.session, &student)
        }
    } else {
        Err(CustomError::InvalidInput {
            field: "marks should not be greater than 100",
        })
    }
}

pub fn show(
    data: State<AppState>,
    student_id: Path<i32>,
) -> Result<Json<Option<Student>>, CustomError> {
    let student: Option<Student> = select_student(&data.session, student_id);
    match student {
        Some(student) => Ok(Json(Some(Student {
            roll_no: student.roll_no,
            marks: student.marks,
            name: student.name.clone(),
        }))),
        None => Err(CustomError::InvalidInput {
            field: "student with this id doesn't exist",
        }),
    }
}

pub fn delete(data: State<AppState>, student_id: Path<i32>) -> Result<&'static str> {
    delete_student(&data.session, student_id)
}

pub fn update(
    data: State<AppState>,
    student: Json<Student>,
    student_id: Path<i32>
) -> Result<&'static str> {
    update_student(&data.session, &student, student_id.into_inner())
}
