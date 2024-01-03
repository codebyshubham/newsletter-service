use std::net::TcpListener;
use sqlx::{PgPool};
use newsletter_service::startup::run;
use newsletter_service::configuration;
use env_logger::{Env};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = configuration::get_configuration().expect("Failed to load configuration!");

    let connection_string = configuration.database.connection_string();

    let connection = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    println!("Server is running on 127.0.0.1:{}", listener.local_addr().unwrap().port());

    run(listener, connection)?.await
}