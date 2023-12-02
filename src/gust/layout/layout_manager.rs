// Define a trait for layout managers
pub trait LayoutManager {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]);
}
