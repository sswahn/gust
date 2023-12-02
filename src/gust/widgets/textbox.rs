use std::rc::Rc;
mod widget;
use widget::Widget;

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
    fn set_styles(&mut self, styles: HashMap<String, String>) {
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
