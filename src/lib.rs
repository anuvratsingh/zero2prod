<<<<<<< HEAD
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
=======
use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
>>>>>>> f7366015145a62a1e6be59f68890f05367911bcc

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
