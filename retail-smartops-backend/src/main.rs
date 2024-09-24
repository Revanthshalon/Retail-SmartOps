use retail_smartops_backend::run_app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run_app(None).await
}
