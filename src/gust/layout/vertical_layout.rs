// Define a trait for layout managers
trait LayoutManager {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]);
}

// Define a basic vertical layout manager
struct VerticalLayout;

impl LayoutManager for VerticalLayout {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]) {
        for widget in widgets {
            widget.render();
        }
    }
}

// Define a basic horizontal layout manager
struct HorizontalLayout;

impl LayoutManager for HorizontalLayout {
    fn arrange_widgets(&self, widgets: &[&dyn Widget]) {
        for widget in widgets {
            widget.render();
        }
    }
}

// Define a basic grid layout manager
struct GridLayout {
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

/*
fn main() {
    // Create instances of widgets
    let label = Label::new("Hello, Rust!");
    let button = Button::new("Click me", Rc::new(|| println!("Button clicked!")));
    let text_box = TextBox::new(Rc::new(|text| println!("Text changed: {}", text)));

    // Arrange widgets using the vertical layout manager
    let vertical_layout = VerticalLayout;
    let widgets: Vec<&dyn Widget> = vec![&label, &button, &text_box];
    println!("Vertical Layout:");
    vertical_layout.arrange_widgets(&widgets);

    // Arrange widgets using the horizontal layout manager
    let horizontal_layout = HorizontalLayout;
    println!("Horizontal Layout:");
    horizontal_layout.arrange_widgets(&widgets);

    // Arrange widgets using the grid layout manager
    let grid_layout = GridLayout::new(2);
    println!("Grid Layout (2 columns):");
    grid_layout.arrange_widgets(&widgets);
}*/
