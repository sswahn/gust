use std::rc::Rc;
mod widget;
use widget::Widget;

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
    fn set_styles(&mut self, styles: HashMap<String, String>) {
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
