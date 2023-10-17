use suspect::{Gender, Suspect};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::error::ErrorBadRequest;
use actix_web::http::StatusCode;
use actix_files::Files;
use serde::Deserialize;
use crate::portrait::make_face;

mod suspect;
mod portrait;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(create_suspect)
            .service(create_suspect_image)
            .service(Files::new("/", "./static").prefer_utf8(true))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy!")
}

#[get("/suspect/{gender}/{name}")]
async fn create_suspect(req: web::Path<CreateSuspectRequest>) -> impl Responder {
    if req.name.trim().len() <= 0 {
        return ErrorBadRequest("Missing name").error_response();
    }

    HttpResponse::Ok().json(Suspect::from_name(&req.name, req.gender))
}

#[get("/suspect/{gender}/{name}/image.png")]
async fn create_suspect_image(req: web::Path<CreateSuspectRequest>) -> impl Responder {
    if req.name.trim().len() <= 0 {
        return ErrorBadRequest("Missing name").error_response();
    }

    let sus = Suspect::from_name(&req.name, req.gender);
    let data = make_face(&sus);
    HttpResponse::build(StatusCode::OK)
        .content_type("image/png")
        .body(data)
}


#[derive(Deserialize)]
struct CreateSuspectRequest {
    name: String,
    gender: Gender,
}

