use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
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

                // Emit a custom button click event
                self.emit_custom_event(CustomEvent::CustomButtonClick);

                false
            }
            _ => false,
        }
    }

    fn handle_keyboard_input(&mut self, input: &KeyboardInput) {
        if let ElementState::Pressed = input.state {
            match input.virtual_keycode {
                Some(VirtualKeyCode::Enter) => {
                    // Simulate a button click when Enter key is pressed
                    self.count += 1;
                    println!("Button clicked: {}", self.count);

                    // Emit a custom button click event
                    self.emit_custom_event(CustomEvent::CustomButtonClick);
                }
                // Add more keyboard navigation as needed
                _ => (),
            }
        }
    }

    fn emit_custom_event(&self, event: CustomEvent) {
        match event {
            CustomEvent::CustomButtonClick => {
                // Handle custom button click event
                println!("Custom button click event triggered!");
            }
            // Add more custom events handling as needed
        }
    }

    fn handle_custom_event(&self, event: CustomEvent) {
        // Handle user-defined custom events
        match event {
            CustomEvent::CustomButtonClick => {
                // Handle custom button click event
                println!("Handling user-defined custom button click event!");
            }
            // Add more custom events handling as needed
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
                Event::UserEvent(user_event) => {
                    // Handle user-defined custom events
                    self.handle_custom_event(*user_event);
                }
                Event::KeyboardInput { input, .. } => {
                    // Handle keyboard input events
                    self.handle_keyboard_input(input);
                }
                _ => (),
            }
        });
    }
}

// CustomEvent enum remains unchanged
enum CustomEvent {
    CustomButtonClick,
    // Add more custom events as needed
}

fn main() {
    let event_loop = EventLoop::new();
    let mut gust = Gust::new();
    gust.run(event_loop);
}
