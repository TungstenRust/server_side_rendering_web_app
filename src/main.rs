use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

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

    println!("Listening on port 8080");
    HttpServer::new(move ||{
        App::new()
            .app_data(handlebars_ref.clone())
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
