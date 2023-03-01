use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout
    );
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // No longer async, given that we don't actually try to connect.
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );
    // We have removed the hardcoded host and port -- they're coming from our settings!
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    // Bubble up the io::Error if we failed to bind the address. Otherwise call .await on our
    // Server
    let listener = TcpListener::bind(address)
        .expect("Failed to bind application");
    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
