const PORT: u16 = 4173;

#[tokio::main]
async fn main() {
    monolith_backend::run(PORT).await;
}
