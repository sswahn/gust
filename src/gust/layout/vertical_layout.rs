mod layout_manager;
use layout_manager::LayoutManager;

// Define a basic vertical layout manager
struct VerticalLayout;

impl LayoutManager for VerticalLayout {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]) {
        for widget in widgets {
            widget.render();
        }
    }
}
