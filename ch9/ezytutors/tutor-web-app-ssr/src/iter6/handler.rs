use argon2::{self, Config};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;
use crate::model::*;
use crate::iter5::state::AppState;
use crate::iter5::errors::EzyTutorError;
use crate::iter5::dbaccess::{get_user_record, post_new_user};

pub async fn show_register_form(tmpl: web::Data<tera::Tera>)
    -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let s = tmpl.render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError(
            "Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.to_string()).await;
    let user_not_found: bool = user.is_err();
    if user_not_found {
        if params.password != params.confirmation {
            ctx.insert("error", "Password do not match");
            ctx.insert("current_username", &username);
            ctx.insert("current_password", &params.password);
            ctx.insert("current_confirmation", &params.confirmation);
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);
            s = tmpl.render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError(
                    "Template error".to_string()))?;
        } else {
            let new_tutor = json!({
                "tutor_name": params.username.to_string(),
                "tutor_pic_url": params.imageurl.to_string(),
                "tutor_profile": params.profile.to_string()
            });
            let awc_client = awc::Client::default();
            let res = awc_client
                .post("http://localhost:3000/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap()
                .body()
                .await?;
            let tutor_response: TutorResponse = serde_json::from_str(
                &std::str::from_utf8(&res)?
            )?;
            s = format!("Congratulations!\
                You can sign in to the site with you login and password.");
            let salt = b"secret_salt";
            let config = Config::default();
            let hash = argon2::hash_encoded(params.password.clone().as_bytes(),
                salt, &config).unwrap();
            let user = User {
                username: username.to_string(),
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash.to_string(),
            };
            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "User id already exists");
        ctx.insert("current_username", &username);
        ctx.insert("current_password", &params.password);
        ctx.insert("current_confirmation", &params.confirmation);
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl.render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError(
                "Template error".to_string()))?;
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
