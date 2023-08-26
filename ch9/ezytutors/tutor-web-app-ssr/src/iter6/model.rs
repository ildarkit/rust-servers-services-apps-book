use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegisterForm {
    pub username: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorSigninForm {
    pub username: String,
    pub password: String, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorResponse {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub tutor_id: Option<i32>,
    pub user_password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewCourse {
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCourseResponse {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: String,
}
