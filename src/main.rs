mod routes;

use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hardcoded `8000` -- it's coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if we failed to bind the address. Otherwise call .await on our
    // Server
    let listener = TcpListener::bind(address)
        .expect("Failed to bind application");
    run(listener)?.await
}
