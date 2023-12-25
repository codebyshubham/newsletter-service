use std::net::TcpListener;
use newsletter_service::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind at random port");

    println!("Server is running on 127.0.0.1:{}", listener.local_addr().unwrap().port());

    run(listener)?.await
}