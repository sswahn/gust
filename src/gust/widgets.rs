use std::rc::Rc;

// Define a trait for common properties and behavior of widgets
trait Widget {
    fn render(&self);
}

// Define a basic label widget
struct Label {
    text: String,
}

impl Label {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Widget for Label {
    fn render(&self) {
        println!("Label: {}", self.text);
    }
}

// Define a basic button widget
struct Button {
    label: String,
    on_click: Rc<dyn Fn()>,
}

impl Button {
    fn new(label: &str, on_click: Rc<dyn Fn()>) -> Self {
        Self {
            label: label.to_string(),
            on_click,
        }
    }

    fn click(&self) {
        (self.on_click)();
    }
}

impl Widget for Button {
    fn render(&self) {
        println!("Button: {}", self.label);
    }
}

// Define a basic text box widget
struct TextBox {
    text: String,
    on_change: Rc<dyn Fn(&str)>,
}

impl TextBox {
    fn new(on_change: Rc<dyn Fn(&str)>) -> Self {
        Self {
            text: String::new(),
            on_change,
        }
    }

    fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        (self.on_change)(&self.text);
    }
}

impl Widget for TextBox {
    fn render(&self) {
        println!("Text Box: {}", self.text);
    }
}

// Define a basic menu item widget
struct MenuItem {
    label: String,
    on_select: Rc<dyn Fn()>,
}

impl MenuItem {
    fn new(label: &str, on_select: Rc<dyn Fn()>) -> Self {
        Self {
            label: label.to_string(),
            on_select,
        }
    }

    fn select(&self) {
        (self.on_select)();
    }
}

impl Widget for MenuItem {
    fn render(&self) {
        println!("Menu Item: {}", self.label);
    }
}

// Define a menu widget
struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    fn select_item(&self, index: usize) {
        if let Some(item) = self.items.get(index) {
            item.select();
        }
    }
}

impl Widget for Menu {
    fn render(&self) {
        println!("Menu:");
        for (index, item) in self.items.iter().enumerate() {
            println!("  {}. {}", index + 1, item.label);
        }
    }
}


/*
fn main() {
    // Create instances of the widgets
    let label = Label::new("Hello, Rust!");
    let button = Button::new("Click me", Rc::new(|| println!("Button clicked!")));
    let text_box = TextBox::new(Rc::new(|text| println!("Text changed: {}", text)));

    // Render the widgets
    label.render();
    button.render();
    text_box.render();

    // Interact with the button and text box
    button.click();
    text_box.set_text("New text");
}*/
