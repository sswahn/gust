// Define a struct for widget styling
struct Styles {
    text_color: Option<String>,
    background_color: Option<String>,
    font_size: Option<f32>,
}

impl Styles {
    fn new(styles: HashMap<String, String>) -> Self {
        Self {
            text_color: styles.get("text_color").cloned(),
            background_color: styles.get("background_color").cloned(),
            font_size: styles.get("font_size").cloned(),
        }
    }
}
