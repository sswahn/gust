// Define a struct for widget styling
struct Style {
    text_color: String,
    background_color: String,
    font_size: u16,
}

impl Style {
    fn new(text_color: &str, background_color: &str, font_size: u16) -> Self {
        Self {
            text_color: text_color.to_string(),
            background_color: background_color.to_string(),
            font_size,
        }
    }
}
