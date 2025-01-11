use actix_web::{get, HttpResponse, HttpServer, App};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("HOLA MUNDO")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
         .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
