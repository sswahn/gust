use winit::{
    event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{Key, ModifiersState},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{WindowBuilder, WindowId},
};
use std::collections::HashMap;
use std::error::Error;

pub struct Gust {
    windows: HashMap<Window>,
    count: usize,
}

impl Gust {
    fn new() -> Self {
        Self {
            windows: HashMap::new(),
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

        self.windows.insert(window.id(), window);
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
                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => {
                        match button {
                            MouseButton::Left => {
                                // Handle left mouse button press
                            }
                            MouseButton::Right => {
                                // Handle right mouse button press
                            }
                    }
                    ElementState::Released => {
                        // Handle mouse button release
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed && !event.repeat {
                        let modifiers = ModifiersState::default();
                        match event.key_without_modifiers().as_ref() {
                            Key::Character("1") => {
                                if modifiers.shift_key() {
                                    println!("Shift + 1 | logical_key: {:?}", event.logical_key);
                                } else {
                                    println!("1");
                                }
                            }
                            Key::Character("t") => {
                                if modifiers.control_key() {
                                    let tabbing_id = self.windows.len() + 1;
                                    let window = WindowBuilder::new()
                                        .with_tabbing_identifier(&tabbing_id)
                                        .build(elwt)
                                        .unwrap();
                                    self.windows.insert(window.id(), window);
                                }
                            }
                        }
                    }
                }
                WindowEvent::MouseWheel { delta, device_id, .. } => {
                    if let Some(window) = self.windows.get_mut(device_id) {
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
