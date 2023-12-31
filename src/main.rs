use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(
                web::scope("/test")
                    .route("/index.html", web::get().to(manual_hello))
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
