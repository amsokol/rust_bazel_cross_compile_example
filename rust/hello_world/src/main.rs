use mimalloc::MiMalloc;
use tokio;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    println!("Hello, world from Tokio!");
}
