mod models;

use std::env;
use self::models::*;
use actix_files::Files;
use actix_web::{http, web, App, Error, HttpServer, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

async fn index(handlebar:web::Data<Handlebars<'_>>) ->HttpResponse{
    let data = json!({
        "project_name": "Certifications",
        "certs": [
            {
                "name": "Professional",
                "image_path":
                "/static/image/cdp.png"
            },
            {
                "name": "Expert",
                "image_path":
                "/static/image/cde.png"
            },
            {
                "name": "Architect",
                "image_path":
                "/static/image/cda.png"
            }
        ]
    });
    let body = handlebar.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL mus be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("Listening on port 8080");
    HttpServer::new(move ||{
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
