use winit::{
    event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{Key, ModifiersState},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{WindowBuilder, WindowId},
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
            .unwrap();

        self.windows.push(Window {
            id: window.id(),
            title: title.to_string(),
            is_minimized: false,
        });
        Ok(())
    }

    fn handle_event(&mut self, event: &Event<()>, elwt: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
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
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed && !event.repeat {
                        match event.key_without_modifiers().as_ref() {
                            Key::Character("1") => {
                                let modifiers = ModifiersState::default();
                                if modifiers.shift_key() {
                                    println!("Shift + 1 | logical_key: {:?}", event.logical_key);
                                } else {
                                    println!("1");
                                }
                            }
                        }
                    }
                }
                WindowEvent::MouseWheel { delta, device_id, .. } => {
                    if let Some(window) = self.windows.iter_mut().find(|w| &w.id == *device_id) {
                        match delta {
                            MouseScrollDelta::LineDelta(x, y) => {
                                let pixels_per_line = 120.0;
                                let mut pos = window.outer_position().unwrap();
                                pos.x += (x * pixels_per_line) as i32;
                                pos.y += (y * pixels_per_line) as i32;
                                window.set_outer_position(pos)
                            }
                            MouseScrollDelta::PixelDelta(p) => {
                                let mut pos = window.outer_position().unwrap();
                                pos.x += p.x as i32;
                                pos.y += p.y as i32;
                                window.set_outer_position(pos)
                            }
                        }
                    }
                }
            },
            Event::UserEvent(user_event) => {
                // Handle other predefined user events if needed
                println!("UserEvent user_event: {:?}", user_event);
            }
        }
    }

    fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        self.create_window(&event_loop, "Main Window").unwrap_or_else(|err| {
            eprintln!("Error creating window: {}", err);
        });
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run(|event, elwt| self.handle_event(&event, elwt)).unwrap_or_else(|err| {
            eprintln!("Error during event loop: {}", err);
        });
    }
}
