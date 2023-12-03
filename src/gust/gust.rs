use winit::{
    event::{Event, WindowEvent, MouseButton, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowId},
    keyboard::KeyCode,
};
use std::error::Error;

pub struct Gust {
    windows: Vec<Window>,
    count: usize,
}

struct Window {
    id: WindowId,
    title: String,
    is_minimized: bool,
}

impl Gust {
    fn new() -> Self {
        Self {
            windows: Vec::new(),
            count: 0,
        }
    }

    fn create_window(&mut self, event_loop: &EventLoop<()>, title: &str) -> Result<(), Box<dyn Error>> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_resizable(true)
            .with_decorations(true)
            .build(&event_loop)
            .unwrap()?;

        self.windows.push(Window {
            id: window.id(),
            title: title.to_string(),
            is_minimized: false,
        });
        Ok(())
    }

    fn handle_event(&mut self, event: &Event<()>, elwt: &EventLoop<()>) {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    },
                    WindowEvent::Resized(_) => {
                        // Handle resize event if needed
                    }
                    WindowEvent::MouseInput { state, button, .. } if *state == ElementState::Pressed => {
                        // Handle mouse button press, e.g., check which button was pressed
                        match button {
                            MouseButton::Left => {
                                // Handle left mouse button press
                            }
                            MouseButton::Right => {
                                // Handle right mouse button press
                            }
                            _ => (),
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let Some(keycode) = event.physical_key {
                            match keycode {
                                KeyCode::A => {
                                    // Handle the 'A' key press
                                }
                                KeyCode::B => {
                                    // Handle the 'B' key press
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                },
            },
            Event::UserEvent(user_event) => {
                // Handle other predefined user events if needed
            }
            _ => (),
        }
    }

    fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        self.create_window(&event_loop, "Main Window").unwrap_or_else(|err| {
            eprintln!("Error creating window: {}", err);
        });
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run(|event, elwt| {
            self.handle_event(&event, &elwt);
        });
    }
}
