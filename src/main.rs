#[macro_use]
extern crate diesel;
use actix_files::Files;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
use awmp::Parts;
use std::collections::HashMap;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Serialize};
use handlebars::Handlebars;
mod models;
mod schema;
use self::schema::certs::dsl::*;
use crate::models::Cert;
use crate::models::NewCert;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    certs: Vec<self::models::Cert>,
}

async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let connection = pool.get()
        .expect("Can't get db connection from pool!");
    let certs_data = web::block(move || { certs.limit(10).load::<Cert>(&connection)
        })
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    let data = IndexTemplateData {
        project_name: "server_side_rendered_my_blogspot".to_string(),
        certs: certs_data,
    };
    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
    }

async fn add(hb:web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error>{
    let body = hb.render("add", &{}).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
async fn add_cert_form(pool: web::Data<DbPool>, mut parts: Parts) -> Result<HttpResponse, Error> {
    let file_path = parts
            .files
            .take("image")
            .pop()
            .and_then(|f| f.persist_in("./static/image").ok())
            .unwrap_or_default();
    let text_fields: HashMap<_, _> = parts.texts.as_pairs().into_iter().collect();
    let connection = pool
        .get()
        .expect("Can't get db connection from pool");
    let new_cert = NewCert {
        name: text_fields.get("name").unwrap().to_string(),
        image_path: file_path.to_string_lossy().to_string()
    };
    web::block(move ||
        diesel::insert_into(certs)
            .values(&new_cert)
            .execute(&connection)
        )
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::SeeOther().append_header(http::header::LOCATION).finish())
}
async fn cert(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>, cert_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Can't get db connection from pool");

    let cert_data = web::block(move || certs.filter(id.eq(cert_id.into_inner())).first::<Cert>(&connection))
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().finish()
        })?;

    let body = hb.render("cert", &cert_data).unwrap();

    Ok(HttpResponse::Ok().body(body))
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
            .data(pool.clone())//-->deprecated insecure
            //.app_data(pool::new(val))
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
            .route("/add", web::get().to(add))
            .route("/add_cert_form", web::post().to(add_cert_form))
            .route("/cert/{id}", web::get().to(cert))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
