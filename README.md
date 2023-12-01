# Gust

A simple GUI for Rust applications.

## Features

- Structured with a modular design for flexibility and easy customization.
- A variety of pre-built widgets, including buttons, labels, text boxes, and menus.
- Flexible layout system to arrange widgets in a structured manner.
- Customizable styling options for widgets and overall GUI appearance.
- Intuitive event handling system for user interactions.
- Simple API for easy integration into Rust applications.
- Built on top of the cross-platform Winit library, ensuring compatibility across major platforms.
- Easily extendable and customizable to meet specific application requirements.


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
