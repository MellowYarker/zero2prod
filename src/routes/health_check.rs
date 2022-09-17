use actix_web::{HttpResponse};

pub async fn health_check() -> HttpResponse {
    // HttpResponder::Ok() gives an HttpResponseBuilder, along w/ a 200 status code.
    // The Builder exposes a great API for building an HttpResponse but we don't need that yet.
    // We could also omit .finish(), since the Builder impl's Responder!
    HttpResponse::Ok().finish()
}
