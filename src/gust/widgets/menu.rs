use std::rc::Rc;
mod widget;
use widget::Widget;

// Define a menu widget
pub struct Menu {
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
    fn set_styles(&mut self, styles: HashMap<String, String>) {
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
