use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::time::Instant;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("I think it's time to be happy")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I am doing well and you?")
}

#[get("/version")]
async fn version() -> impl Responder {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    HttpResponse::Ok().body(VERSION)
}

fn calculate_sum(limit: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..limit {
        sum += i;
    }
    sum
}

#[get("/iambest/{limit}")]
async fn iambest(req: HttpRequest) -> HttpResponse {
    let start = Instant::now();

    let limit: u64 = req.match_info().get("limit").unwrap().parse().unwrap();
    let sum = calculate_sum(limit);

    let duration = start.elapsed();
    HttpResponse::Ok().body(format!(
        "duration: {:?}, limit: {}, sum: {}",
        duration, limit, sum
    ))
}

#[get("/iambest/{limit}/{times}")]
async fn iambest_times(req: HttpRequest) -> HttpResponse {
    let start = Instant::now();

    let limit: u64 = req.match_info().get("limit").unwrap().parse().unwrap();
    let times: u64 = req.match_info().get("times").unwrap().parse().unwrap();

    let mut sum: u64 = 0;
    for _ in 0..times {
        sum = calculate_sum(limit);
    }

    let duration = start.elapsed();
    HttpResponse::Ok().body(format!(
        "duration: {:?}, limit: {}, sum: {}",
        duration, limit, sum
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Rust Actix-web server started at 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(healthcheck)
            .service(version)
            .service(iambest)
            .service(iambest_times)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
