use app::run;

pub mod window;
pub mod app;
pub mod objects;
pub mod asset;
pub mod engine;
pub mod util;
pub mod scene;
pub mod transform;


fn main() {
    pollster::block_on(run());
}