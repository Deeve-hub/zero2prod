use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Bubble up the io::Error if we fail to bind
    // Otherwise call await
    run()?.await
}
