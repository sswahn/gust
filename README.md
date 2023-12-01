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
use gust::{Gust, Button, TextBox, Label};

struct MyApp {
    gust: Gust,
}

// Define a button click callback
let on_button_click = || {
    println!("Button Clicked");
};

// Define a button click callback
let on_text_change = |text| {
    println!("Text Box Changed: {}", text)
};


impl MyApp {
    fn new() -> Self {
        Self { gust: Gust::new() }
    }

    fn run(&mut self) {
        self.gust.create_window("Main Window");

        // Create a button with a closure
        let button = Button::new("Click Me", on_button_click);
        let text_box = TextBox::new(on_text_change);
        let label = Label::new("Hello, Rust!");

        // Create custom styles
        let mut button_custom_styles = HashMap::new();
        button_custom_styles.insert("background_color", "black");
        button_custom_styles.insert("text_color", "white");
        button_custom_styles.insert("font_size", 16);

        // Apply custom styles
        let button_styles = Styles::new(button_custom_styles)
        button.set_styles(button_styles);

        // Add widgets to the layout
        self.gust.render(button);
        self.gust.render(text_box);
        self.gust.render(label);
    }
}

fn main() {
    let mut app = MyApp::new();
    app.run();
}
```

## License

This project is [MIT License](LICENSE).
