use std::net::TcpListener;
use newsletter_service::startup::run;
use newsletter_service::configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to load configuration!");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    println!("Server is running on 127.0.0.1:{}", listener.local_addr().unwrap().port());

    run(listener)?.await
}