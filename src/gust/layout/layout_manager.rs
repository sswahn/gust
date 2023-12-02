// Define a trait for layout managers
trait LayoutManager {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]);
}
