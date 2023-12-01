use std::rc::Rc;

// break widgets out into own files, and create default styles.

// Define a trait for common properties and behavior of widgets
trait Widget {
    fn render(&self);
}

// Define a basic label widget
struct Label {
    text: String,
    styles: Option<Styles>
}

impl Label {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            styles: None,
        }
    }
    fn set_style(&mut self, styles: Styles) {
        self.styles = Some(styles);
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
    styles: Option<Styles>,
    on_click: Rc<dyn Fn()>,
}

impl Button {
    fn new(label: &str, on_click: Rc<dyn Fn()>) -> Self {
        Self {
            label: label.to_string(),
            styles: None,
            on_click,
        }
    }
    fn set_styles(&mut self, styles: Styless) {
        self.style = Some(styles);
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
    styles: Option<Styles>,
    on_change: Rc<dyn Fn(&str)>,
}

impl TextBox {
    fn new(on_change: Rc<dyn Fn(&str)>) -> Self {
        Self {
            text: String::new(),
            styles: None,
            on_change,
        }
    }
    fn set_styles(&mut self, styles: Styles) {
        self.styles = Some(styles);
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
    styles: Option<Styles>,
    on_select: Rc<dyn Fn()>,
}

impl MenuItem {
    fn new(label: &str, on_select: Rc<dyn Fn()>) -> Self {
        Self {
            label: label.to_string(),
            styles: None,
            on_select,
        }
    }
    fn set_styles(&mut self, styles: Styles) {
        self.styles = Some(styles);
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
    styles: Option<Styles>,
}

impl Menu {
    fn new() -> Self {
        Self { 
            items: Vec::new(),
            styles: None,
        }
    }
    fn set_styles(&mut self, styles: Styles) {
        self.styles = Some(styles);
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
