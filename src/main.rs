use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/{name}", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
