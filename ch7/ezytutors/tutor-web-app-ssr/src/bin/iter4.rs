use actix_files as fs;
use tera::Tera;
use serde::{Serialize, Deserialize};
use actix_web::{error, web, 
    App, Error, HttpResponse, HttpServer, Result};
use awc::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>)
-> Result<HttpResponse, Error> {
    let client = Client::default();
    let response = client
        .get("http://localhost:3000/tutors/")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();
    let str_list = std::str::from_utf8(&response.as_ref()).unwrap();
    let tutor_list: Vec<Tutor> = serde_json::from_str(str_list)
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutor_list);
    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| 
            error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html").body(rendered_html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    println!("Listening on: 127.0.0.1:8080");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter4/**/*"
        ))
        .unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static")
                .show_files_listing())
            .service(web::resource("/tutors")
                .route(web::get().to(handle_get_tutors)))
    );
}
