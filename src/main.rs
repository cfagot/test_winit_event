use std::sync::Arc;
use std::time::Instant;

use render_context::{RenderContext, RenderSurface};
use wgpu::PresentMode;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

mod render_context;

struct TestApp {
    start_time: Instant,
    last_render: Instant,
    render_context: RenderContext,
    window: Option<Arc::<Window>>,
    surface: Option<RenderSurface<'static>>,
}

impl TestApp {
    fn new() -> Self {
        let render_context = RenderContext::new();
        Self {
            start_time: Instant::now(),
            last_render: Instant::now(),
            render_context,
            window: None,
            surface: None,
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
        let window = Arc::new(window);
        window.set_visible(true);

        // let size = window.inner_size();
        // let surface = pollster::block_on(self.render_context.create_surface(window.clone(), size.width, size.height, PresentMode::Fifo));
        self.window = Some(window);
        // self.surface = Some(surface);
        println!("Window created at {:?}", self.start_time.elapsed());
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
            WindowEvent::RedrawRequested => {
                println!("WindowEvent::RedrawRequested at {:?}", self.start_time.elapsed());

                // let surface_texture = self.surface.as_ref().unwrap().surface.get_current_texture().unwrap();
                // surface_texture.present();
                self.last_render = Instant::now();
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
        if self.last_render.elapsed().as_secs_f32() < 0.03 {
            return;
        }
        println!("About to wait at {:?}", self.start_time.elapsed());
        self.window.as_ref().unwrap().request_redraw();
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

