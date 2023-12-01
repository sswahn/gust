use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
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

enum CustomEvent {
    CustomButtonClick,
    // Add more custom events as needed
}

impl Gust {
    fn new() -> Self {
        Self {
            windows: Vec::new(),
            count: 0,
        }
    }

    fn create_window(&mut self, event_loop: &EventLoop<()>, title: &str) {
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
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match &event {
                Event::WindowEvent { event, window_id } => {
                    // Handle window events
                }
                _ => (),
            }
        });
    }

    fn handle_window_event(&mut self, event: &WindowEvent, window_id: winit::window::WindowId) -> bool {
        match event {
            WindowEvent::CloseRequested if self.is_main_window(window_id) => true,
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

    fn run(&mut self) {
        let event_loop = EventLoop::new();
        self.create_window(&event_loop, "Main Window");

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
                _ => (),
            }
        });
    }

    fn is_main_window(&self, window_id: winit::window::WindowId) -> bool {
        self.windows.iter().any(|window| window.id == window_id)
    }
}
