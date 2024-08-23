use actix_web::{web, App, HttpServer, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::fs;
use std::path::Path;

async fn index() -> HttpResponse {
    let index_path = Path::new("index.html");
    match fs::read_to_string(index_path) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read index.html"),
    }
}

async fn img() -> HttpResponse {
    let img_path = Path::new("img.png");
    match fs::read(img_path) {
        Ok(image_data) => HttpResponse::Ok().content_type("image/jpeg").body(image_data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read image"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("server.key", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("server.crt").unwrap();

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/img.png", web::get().to(img))
    })
    .bind_openssl("0.0.0.0:8081", acceptor)?
    .run()
    .await
}