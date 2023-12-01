use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowBuilderExtWindows,
    window::WindowBuilder,
};

struct Gust {
    windows: Vec<Window>,
    count: usize,
    custom_event_handlers: HashMap<CustomEvent, Box<dyn Fn()>>,
}

struct Window {
    id: winit::window::WindowId,
    title: String,
    is_minimized: bool,
}

enum CustomEvent {
    ButtonClick,
    TextChange(String),
    // Add more custom events as needed
}

impl Gust {
    fn new() -> Self {
        Self {
            windows: Vec::new(),
            count: 0,
            custom_event_handlers: HashMap::new(),
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

        let windows_ref = &mut self.windows;
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match &event {
                Event::WindowEvent { event, window_id } => {
                    // Handle window events
                    if let WindowEvent::CloseRequested = event {
                        // Remove the closed window
                        windows_ref.retain(|window| window.id != *window_id);

                        // If the last window is closed, exit the application
                        if windows_ref.is_empty() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => (),
            }
        });
        Ok(())
    }

fn handle_window_event(&mut self, event: &WindowEvent, window_id: winit::window::WindowId) -> bool {
    match event {
        WindowEvent::CloseRequested if self.is_main_window(window_id) => true,
        WindowEvent::Resized(_) => {
            // Handle resize event if needed. handle this.
        }
        WindowEvent::MouseInput { state, button, .. } if matches!(state, winit::event::ElementState::Pressed) => {
            self.handle_mouse_click();
        }
        _ => (),
    }
}

fn handle_mouse_click(&mut self) {
    self.count += 1;
    println!("Button clicked: {}", self.count);

    // Emit a custom button click event
    self.emit_custom_event(CustomEvent::CustomButtonClick);
}


    fn handle_event(&self, event: CustomEvent) {
        // Handle user-defined custom events
        if let Some(handler) = self.custom_event_handlers.get(&event) {
            handler();
        }
    }

    fn add_event_handler(&mut self, event: CustomEvent, handler: Box<dyn Fn()>) {
        self.custom_event_handlers.insert(event, handler);
    }

    /* Example of adding a custom event handler
    gust.add_event_handler(CustomEvent::ButtonClick, Box::new(|| {
        println!("Handling custom button click event!");
    }));
    */


    fn run(&mut self) {
        let event_loop = EventLoop::new();
        self.create_window(&event_loop, "Main Window").unwrap_or_else(|err| {
            eprintln!("Error creating window: {}", err);
        });
    
        self.run_event_loop(event_loop);
    }

    fn run_event_loop(&mut self, mut event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match &event {
                Event::WindowEvent { event, window_id } => {
                    self.handle_window_event(event, *window_id)
                    if self.windows.is_empty() {
                        *control_flow = ControlFlow::Exit; // If the last window is closed, exit the application
                    }
                }
                Event::UserEvent(user_event) => {
                    self.handle_custom_event(*user_event); // Handle user-defined custom events
                }
                _ => (),
            }
        });
    }

    fn is_main_window(&self, window_id: winit::window::WindowId) -> bool {
        self.windows.iter().any(|window| window.id == window_id)
    }
}
