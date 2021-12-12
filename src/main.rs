use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn test_service() -> impl Responder {
    HttpResponse::Ok().body("Hello from test_service!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test_service)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
