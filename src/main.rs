mod server;

#[async_std::main]
async fn main() -> tide::Result<()> {
    server::start().await
}
