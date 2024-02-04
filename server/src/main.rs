use async_std::main;
use human_panic::setup_panic;

#[main]
async fn main() {
    setup_panic!();
}
