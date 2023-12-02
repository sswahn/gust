use std::rc::Rc;
mod widget;
use widget::Widget;

// Define a basic button widget
pub struct Button {
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
    fn set_styles(&mut self, styles: HashMap<String, String>) {
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
