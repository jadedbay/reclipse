# Reclipse

WIP Rust game engine.

## Usage

First, clone the repository from GitHub. Then, include it in your `Cargo.toml` dependencies as follows:

[dependencies]
reclipse = { path = "path/to/reclipse" }

Then, in your main.rs file, use the library as follows:

```rust
use reclipse::{window::Window, app::App};

#[tokio::main]
async fn main() {
    let window = Window::new();
    let app = App::new(&window).await;

    reclipse::app::run(window, app).await
}
```

