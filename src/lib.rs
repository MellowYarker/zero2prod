pub mod configuration;
pub mod routes;
pub mod startup;

/*
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    // HttpResponder::Ok() gives an HttpResponseBuilder, along w/ a 200 status code.
    // The Builder exposes a great API for building an HttpResponse but we don't need that yet.
    // We could also omit .finish(), since the Builder impl's Responder!
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword.
// We have no .await call, so it is not needed anymore!
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
    // No .await here!
    Ok(server)
}
*/
