use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct ResponseData {
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fn receive_string(payload: String) -> Result<HttpResponse, actix_web::Error> {
        println!("Received string: {}", payload);

        // Creating a ResponseData struct and serializing it to JSON
        let response_data = ResponseData {
            message: String::from("String received by the server."),
        };
        Ok(HttpResponse::Ok().body(format!("go fuck yourself nigger {:?}", response_data)))
    }

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default() // Allow all origins by default
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_header("content-type")
                    .max_age(3600), // Cache preflight OPTIONS requests for 1 hour
            )
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello, world!") }),
            )
            .route(
                "/calculate",
                web::post().to(|payload: String| async move { receive_string(payload) }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
