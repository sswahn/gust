use std::rc::Rc;
mod widget;
use widget::Widget;


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
    fn set_style(&mut self, styles: HashMap<String, String>) {
        self.styles = Some(styles);
    }
}

impl Widget for Label {
    fn render(&self) {
        println!("Label: {}", self.text);
    }
}
