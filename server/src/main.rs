use human_panic::setup_panic;
use tokio::main;

#[main]
async fn main() {
    setup_panic!();
}
