mod layout_manager;
use layout_manager::LayoutManager;

// Define a basic grid layout manager
pub struct GridLayout {
    columns: usize,
}

impl GridLayout {
    fn new(columns: usize) -> Self {
        Self { columns }
    }
}

impl LayoutManager for GridLayout {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]) {
        for (index, widget) in widgets.iter().enumerate() {
            widget.render();
            if (index + 1) % self.columns == 0 {
                println!(); // Start a new row
            }
        }
    }
}
