use actix_files as fs;
use tera::Tera;
use serde::{Serialize, Deserialize};
use actix_web::{error, web, 
    App, Error, HttpResponse, HttpServer, Result};

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>)
-> Result<HttpResponse, Error> {
    let tutors: Vec<Tutor> = vec![
        Tutor {
            name: String::from("Tutor 1"),
        },
        Tutor {
            name: String::from("Tutor 2"),
        },
        Tutor {
            name: String::from("Tutor 3"),
        },
        Tutor {
            name: String::from("Tutor 4"),
        },
    ];
    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);
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
            "/static/iter3/**/*"
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
