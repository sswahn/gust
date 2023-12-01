// Define a struct for widget styling
struct Style {
    text_color: String,
    background_color: String,
    font_size: String,
}

impl Style {
    fn new(styles: HashMap<String, String>) -> Self {
        Self {
            text_color: styles.text_color,
            background_color: styles.background_color,
            font_size: styles.font_size,
        }
    }
}
