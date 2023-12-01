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
