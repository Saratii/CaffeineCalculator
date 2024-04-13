use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn index() -> impl Responder {
    "Hello, world!"
}

async fn receive_string(payload: String) -> impl Responder {
    println!("Received string: {}", payload);
    HttpResponse::Ok().body("String received by the server.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/send-string", web::post().to(receive_string))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}