mod tokens;
mod ast;
mod eval;
mod math;
mod graph;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use crate::ast::build_ast;
use crate::eval::evaluate_ast;
use crate::graph::graph;
use crate::tokens::{Token, tokenize};

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
        let tokens = tokenize(payload.0.text.clone());
        match tokens{
            Ok(mut tokens) => {
                if tokens.len() == 1 && tokens[0] == Token::Help {
                    println!("Help\nSupports Math Input: 3+4(2 + 9)*(-1)^3\nTrigonometry: sin(t) cos(t) tan(t) asin(t) acos(t) atan(t) sec(t) csc(t) cot(n)\nStatistics: sum(n1, n2) avg(n1, n2) std(n1, n2)\nOther: ln(t) factorial(n)");
                    return Ok(web::Json(ResponseData{message: "Help\nSupports Math Input: 3+4(2 + 9)*(-1)^3\nTrigonometry: sin(t) cos(t) tan(t) asin(t) acos(t) atan(t) sec(t) csc(t) cot(n)\nStatistics: median(n1, n2) sum(n1, n2) avg(n1, n2) std(n1, n2) max(n1, n2) min(n1, n2)\nOther: ln(t) factorial(n)".to_string()}));
                } else if tokens.len() > 0 && tokens[0] == Token::Graph {
                    tokens.pop_front();
                    tokens.pop_back();
                    let points = graph(tokens);
                    return Ok(web::Json(ResponseData {
                        message: points.iter().map(|x| x.to_string()).collect(),
                    }));
                }
                let ast = build_ast(tokens);
                match ast {
                    Ok(ast) => {
                        let val = evaluate_ast(ast);
                        match val {
                            Ok(val) => {
                                println!("Responding with: {}", val);
                                Ok(web::Json(ResponseData{message: (payload.0.text +  " = " + val.to_string().as_str()).to_string()}))
                            },
                            Err(e) => {
                                println!("Failed Eval: {}", e);
                                println!("Responding with: {}", e);
                                Ok(web::Json(ResponseData{message: payload.0.text + ": " + &e}))
                            },
                        }
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