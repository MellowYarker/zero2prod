use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;
use sqlx::PgPool;
use std::net::TcpListener;


// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword.
// We have no .await call, so it is not needed anymore!
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // We wrap the pool in an Arc since it must be cloneable.
    // Why must it be cloneable? Well HttpServer::new() expects the closure to return
    // an instance of an App. Why? So that each thread (worker) can have one.
    //
    // Actually, we can wrap it in web::Data because it's really just a wrapper for an Arc.
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`.
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state.
            .app_data(db_pool.clone())
        })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
