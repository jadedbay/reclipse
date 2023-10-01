use app::run;

pub mod window;
pub mod app;
pub mod objects;
pub mod asset;
pub mod engine;
pub mod util;
pub mod scene;
pub mod transform;

#[tokio::main]
async fn main() {
    run().await
}