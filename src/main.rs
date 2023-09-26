use app::run;

pub mod window;
pub mod app;
pub mod objects;
pub mod asset;
pub mod engine;
pub mod util;


fn main() {
    pollster::block_on(run());
}