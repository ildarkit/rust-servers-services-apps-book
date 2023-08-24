use crate::dbaccess::tutor::*;
use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, UpdateTutor};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>)
    -> Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    get_tutor_details_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn post_new_tutor(
    new_tutor: web::Json<NewTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_tutor_db(&app_state.db, new_tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    update_tutor_details_db(&app_state.db, tutor_id,
        update_tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    delete_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_tutors_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url)
            .await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let resp = get_all_tutors(app_state)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_tutor_details_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url)
            .await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_tutor_details(app_state, tutor_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK); 
    }

    #[ignore]
    #[actix_rt::test]
    async fn post_tutor_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url)
            .await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor = NewTutor{
            tutor_name: "Bob".to_string(),
            tutor_pic_url: "".to_string(),
            tutor_profile: "".to_string(),
        };
        let new_tutor = web::Json(tutor);
        let resp = post_new_tutor(new_tutor, app_state)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_tutor_details_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url)
            .await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor = UpdateTutor{
            tutor_name: Some("Bob".to_string()),
            tutor_pic_url: None,
            tutor_profile: None,
        };
        let update_tutor = web::Json(tutor);
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = update_tutor_details(app_state, tutor_id, update_tutor)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK); 
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_tutor_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url)
            .await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        }); 
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = delete_tutor(app_state, tutor_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
