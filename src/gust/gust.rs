use std::collections::HashMap;
use std::error::Error;
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    event::keyboard::VirtualKeyCode,  // Import VirtualKeyCode from the keyboard module
    platform::windows::WindowBuilderExtWindows,
    window::WindowBuilder,
};

struct Gust {
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

    fn create_window(&mut self, event_loop: &mut EventLoop<()>, title: &str) -> Result<(), Box<dyn Error>> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_resizable(true)
            .with_decorations(true)
            .build(event_loop)
            .expect("Failed to create window");

        let window_id = window.id();
        self.windows.push(Window {
            id: window_id,
            title: title.to_string(),
            is_minimized: false,
        });
        Ok(())
    }


    fn handle_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, window_id, .. } => match event {
                WindowEvent::CloseRequested => {
                    self.windows.retain(|window| window.id != *window_id);
                    if self.windows.is_empty() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                WindowEvent::Resized(_) => {
                    // Handle resize event if needed
                }
                WindowEvent::MouseInput { state, button, .. } if *state == ElementState::Pressed => {
                    self.handle_mouse_click();
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        *control_flow = ControlFlow::Exit;
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
        let event_loop = EventLoop::new();
        self.create_window(&event_loop, "Main Window").unwrap_or_else(|err| {
            eprintln!("Error creating window: {}", err);
        });
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            self.handle_event(&event, control_flow);
        });
    }
}
