mod tokens;
mod ast;
mod eval;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use crate::ast::build_ast;
use crate::eval::evaluate_ast;
use crate::tokens::tokenize;

#[derive(Serialize, Debug)]
struct ResponseData {
    message: String,
}

#[derive(Deserialize)]
struct Request {
    text: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fn receive_string(payload: Json<Request>) -> Result<impl Responder, Error> {
        println!("Received string: {}", payload.0.text);
        let tokens = tokenize(payload.0.text);
        match tokens{
            Ok(tokens) => {
                let ast = build_ast(tokens);
                match ast {
                    Ok(ast) => {
                        let val = evaluate_ast(ast);
                        println!("Responding with: {}", val);
                        Ok(web::Json(ResponseData{message: val.to_string()}))
                    },
                    Err(e) => {
                        println!("Failed Build AST: {}", e);
                        println!("Responding with: {}", e);
                    Ok(web::Json(ResponseData{message: e}))
                    }, 
                }
                
            },
            Err(e) => {
                println!("Failed Tokenize: {}", e);
                println!("Responding with: {}", e);
                Ok(web::Json(ResponseData{message: e}))
            },
        }
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
                web::post().to(|payload: Json<Request>| async move { receive_string(payload) }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}