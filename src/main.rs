//These actix-web types and traits will let us 
//create a web server rapidly

use actix_web::{get, post, web, App, HttpResponse,
    HttpServer, Responder };
//first off, let us create our routes

#[get("/")]//macro for route handling functions
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to icheck, the worlds' most advanced recon server!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await  
    
}