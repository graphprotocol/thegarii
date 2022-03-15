use thegarii::Opt;

#[tokio::main]
async fn main() {
    Opt::exec().await.expect("thegraii crashed")
}
