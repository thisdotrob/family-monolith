const PORT: u16 = 443;

#[tokio::main]
async fn main() {
    monolith_backend::run(PORT).await;
}
