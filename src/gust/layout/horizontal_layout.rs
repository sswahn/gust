mod layout_manager;
use layout_manager::LayoutManager;

// Define a basic horizontal layout manager
pub struct HorizontalLayout;

impl LayoutManager for HorizontalLayout {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]) {
        for widget in widgets {
            widget.render();
        }
    }
}
