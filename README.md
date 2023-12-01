# Gust

A simple GUI for Rust applications.

## Features

- Create resizable and decorated windows.
- Handle window events such as resize, close, mouse input, and keyboard input.
- Maintain multiple windows and exit the application when all windows are closed.  

## Usage

```rust
// Application using Gust
use gust::Gust;

struct MyApp {
    gust: Gust,
}

impl MyApp {
    fn new() -> Self {
        Self { gust: Gust::new() }
    }

    fn run(&mut self) {
        self.gust.create_window("Main Window");
        // Add more Gust framework usage...
    }
}

fn main() {
    let mut app = MyApp::new();
    app.run();
}
```

## License

This project is [MIT License](LICENSE).
