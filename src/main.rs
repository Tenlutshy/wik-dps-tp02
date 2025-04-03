use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use serde_json::json;
use std::collections::HashMap;
use std::env;

#[get("/ping")]
async fn get_ping(request: HttpRequest) -> impl Responder {
    let header: HashMap<String, String> = request
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().unwrap_or("Invalid UTF-8").to_string(),
            )
        })
        .collect();

    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!(header))
}

async fn get_other() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PING_LISTEN_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .service(get_ping)
            .default_service(web::to(get_other))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
