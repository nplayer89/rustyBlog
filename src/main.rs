use rusty_blog::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
