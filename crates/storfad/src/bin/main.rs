#[tokio::main]
async fn main() {
    let exit_code = storfad::server().await;
    std::process::exit(exit_code);
}
