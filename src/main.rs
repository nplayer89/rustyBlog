use std::net::TcpListener;

use rusty_blog::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port");
    run(listener)?.await
}
