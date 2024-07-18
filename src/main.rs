use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};


struct TestApp {
    start_time: Instant,
    window: Option<Window>,
}

impl TestApp {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            window: None,
        }
    }
}

#[allow(unused_variables)]
impl ApplicationHandler for TestApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let min_window_size = winit::dpi::LogicalSize::new(400, 400);
        let window_attributes = winit::window::Window::default_attributes()
            .with_title("Tester")
            .with_resizable(true)
            .with_min_inner_size(min_window_size);

        let window = event_loop.create_window(window_attributes).unwrap();
        window.set_visible(true);
        self.window = Some(window);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                println!("WindowEvent::KeyboardInput at {:?}: {:?}", self.start_time.elapsed(), event);
                if let KeyEvent {physical_key: PhysicalKey::Code(KeyCode::Escape), ..} = event {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
    
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
    }
    
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
    }
    
    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            winit::event::DeviceEvent::Key(key) => {
                println!("DeviceEvent::Key at {:?}: {:?}", self.start_time.elapsed(), key);
            }
            _ => {}
        }
    }
    
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    }
    
    
    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
    }
    
    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
    }
}

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run_app(&mut TestApp::new())
}

