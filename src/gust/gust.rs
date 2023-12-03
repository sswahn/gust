use std::error::Error;
use winit::event::VirtualKeyCode;
//use winit::platform::windows::WindowBuilderExtWindows;
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton}, // where is MouseButton being handled?
    event_loop::{ControlFlow, EventLoop},
    //event::keyboard::VirtualKeyCode,  // Import VirtualKeyCode from the keyboard module
   // platform::windows::WindowBuilderExtWindows,
    window::WindowBuilder,
};

pub struct Gust {
    windows: Vec<Window>,
    count: usize,
}

struct Window {
    id: winit::window::WindowId,
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
            .expect("Failed to create window");

        let window_id = window.id();
        self.windows.push(Window {
            id: window_id,
            title: title.to_string(),
            is_minimized: false,
        });
        Ok(())
    }


    fn handle_event(&mut self, event: &Event<()>, window: &Window, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, window_id, .. } => match event {
                WindowEvent::CloseRequested => {
                    window.exit();
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
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match keycode {
                            VirtualKeyCode::A => {
                                // Handle the 'A' key press
                            }
                            VirtualKeyCode::B => {
                                // Handle the 'B' key press
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
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
        event_loop.run(move |event, window, control_flow| {
            *control_flow = ControlFlow::Wait;
            self.handle_event(&event, &window, control_flow);
        });
    }
}
