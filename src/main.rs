use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/version")]
async fn ver() -> impl Responder {
    HttpResponse::Ok().body("0.2.0")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I am doing well and you?")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Rust Actix-web server started at 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(healthcheck)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
