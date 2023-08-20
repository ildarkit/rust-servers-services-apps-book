use crate::models::course::*;
use crate::errors::EzyTutorError;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32
) -> Result<Vec<Course>, EzyTutorError> {
    let course_rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM
        ezy_course_c6 where tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await?;
    Ok(course_rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    let course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM
        ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id, course_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse)
    -> Result<Course, EzyTutorError> {
    let course_row = sqlx::query_as!(
        Course,
        "insert into ezy_course_c6 (
        tutor_id, course_name, course_description, course_duration,
        course_level, course_format, course_language, course_structure,
        course_price) values($1, $2, $3, $4, $5, $6, $7, $8, $9)
        returning tutor_id, course_id, course_name, course_description,
        course_duration, course_level, course_format, course_language,
        course_structure, course_price, posted_time",
        new_course.tutor_id, new_course.course_name,
        new_course.course_description, new_course.course_duration,
        new_course.course_level, new_course.course_format,
        new_course.course_language, new_course.course_structure,
        new_course.course_price)
        .fetch_one(pool)
        .await?;
    Ok(course_row)
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    let course_row = sqlx::query!(
        "DELETE FROM ezy_course_c6 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
        .execute(pool)
        .await?;
    Ok(format!("Deleted {:#?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| EzyTutorError::NotFound(
        "Course id not found".into()))?;
    let name: String = if let Some(name) = update_course.course_name {
        name
    } else {
        current_course_row.course_name
    };
    let description: String = match update_course.course_description {
        Some(desc) => desc,
        None => current_course_row
            .course_description.unwrap_or_default(),
    };
    let format: String = match update_course.course_format {
        Some(format) => format,
        None => current_course_row
            .course_format.unwrap_or_default(),
    };
    let structure: String = match update_course.course_structure {
        Some(structure) => structure,
        None => current_course_row
            .course_structure.unwrap_or_default(),
    };
    let duration: String = match update_course.course_duration {
        Some(duration) => duration,
        None => current_course_row
            .course_duration.unwrap_or_default(),
    };
    let level: String = match update_course.course_level {
        Some(level) => level,
        None => current_course_row
            .course_level.unwrap_or_default(),
    };
    let language: String = match update_course.course_language {
        Some(language) => language,
        None => current_course_row
            .course_language.unwrap_or_default(),
    };
    let price = match update_course.course_price {
        Some(price) => price,
        None => current_course_row
            .course_price.unwrap_or_default(),
    };
    
    let course_row = sqlx::query_as!(
        Course,
        "UPDATE ezy_course_c6 SET course_name = $1,
        course_description = $2, course_format = $3,
        course_structure = $4, course_duration = $5,
        course_price = $6, course_language = $7,
        course_level = $8 WHERE tutor_id = $9 AND course_id = $10
        RETURNING tutor_id, course_id, course_name, course_description,
        course_duration, course_level, course_format, course_language,
        course_structure, course_price, posted_time",
        name, description, format, structure, duration, price,
        language, level, tutor_id, course_id
    )
        .fetch_one(pool)
        .await;
    course_row.map_or(
        Err(EzyTutorError::NotFound("Course id not found".into())),
        |course| Ok(course)
    )
}
