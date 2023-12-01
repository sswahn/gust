use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowBuilderExtWindows,
    window::WindowBuilder,
};

struct Gust {
    count: usize,
}

impl Gust {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn handle_window_event(&mut self, event: &WindowEvent, window_id: winit::window::WindowId) -> bool {
        match event {
            WindowEvent::CloseRequested if window_id == window_id => true,
            WindowEvent::Resized(_) => {
                // Handle resize event if needed
                false
            }
            WindowEvent::MouseInput { state, button, .. } if state == winit::event::ElementState::Pressed => {
                // Handle mouse click event if needed
                self.count += 1;
                println!("Button clicked: {}", self.count);
                false
            }
            _ => false,
        }
    }

    fn run(&mut self, event_loop: EventLoop<()>) {
        let window = WindowBuilder::new()
            .with_title("Gust: A Rust GUI")
            .with_resizable(true) // Allow resizing
            .with_decorations(true) // Show window decorations (including close button)
            .build(&event_loop)
            .expect("Failed to create window");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match &event {
                Event::WindowEvent { event, window_id } => {
                    if self.handle_window_event(event, *window_id) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut gust = Gust::new();
    gust.run(event_loop);
}

/*
reating a custom GUI framework involves building a set of abstractions and components to handle various aspects of GUI development. Below are some features you might consider adding to your custom Rust GUI framework:

Widgets and Layouts:

Define a set of basic widgets (buttons, labels, text boxes, etc.).
Implement layout managers to arrange widgets on the screen (e.g., grid layout, stack layout).
Event Handling:

Extend event handling to support various user interactions (clicks, keypresses, etc.).
Implement a flexible event system that allows users to define custom events.
Styling and Theming:

Create a styling system for widgets, allowing users to customize the appearance of their applications.
Implement a theming mechanism to switch between different visual styles.
Drawing and Graphics:

Develop a graphics API or integrate with an existing graphics library for custom drawing.
Support custom shapes, images, and other graphical elements.
Animations:

Integrate animation capabilities for smooth transitions and effects.
Allow users to define animations for widget properties.
Window Management:

Extend window management to support multiple windows and dialogs.
Implement window resizing, minimizing, and maximizing.
Internationalization (i18n) and Localization (l10n):

Add support for internationalization and localization of text in the UI.
Allow users to easily switch between different languages.
Accessibility:

Implement accessibility features, making the GUI usable for people with disabilities.
Provide keyboard navigation, screen reader support, and other accessibility enhancements.
File Dialogs:

Create file dialogs for opening and saving files.
Allow users to browse and select files and directories.
Clipboard Integration:

Implement clipboard support for copying and pasting text or data between different parts of the application.
Custom Drawing and Rendering:

Allow users to implement custom drawing routines for specialized use cases.
Support custom rendering pipelines for advanced graphics effects.
Documentation and Examples:

Provide thorough documentation for using the framework.
Include examples and sample applications to demonstrate different features.
*/
