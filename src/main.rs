use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/api/is-odd/{number}")]
async fn index(path: web::Path<i64>) -> impl Responder {
    let number = path.into_inner();
    let is_odd = number % 2 != 0;
    let explanation = if is_odd {
        format!("{} is odd because it's not divisible by 2.", number)
    } else {
        format!("{} is even because it's divisible by 2.", number)
    };

    let response = json!({
        "number": number,
        "is_odd": is_odd,
        "explanation": explanation
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let bind_address = "127.0.0.1:8080";
    log::info!("Starting server at http://{}", bind_address);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
    })
    .bind(bind_address)?
    .workers(8)
    .run()
    .await
}
