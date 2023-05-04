use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Bubble up the io::Error if we fail to bind
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    // Otherwise call await
    run(listener)?.await
}
