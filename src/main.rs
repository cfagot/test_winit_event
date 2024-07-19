use std::f32::consts::TAU;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use render_context::{RenderContext, RenderSurface};
use simple_render::SimpleRender;
use wgpu::PresentMode;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

mod render_context;
mod simple_render;

struct TestApp {
    start_time: Instant,
    last_render: Instant,
    last_update: Instant,
    render_context: RenderContext,
    window: Option<Arc::<Window>>,
    surface: Option<RenderSurface<'static>>,
    simple_render: Option<SimpleRender>,
    rotation_rate: [f32; 4],
}

impl TestApp {
    fn new() -> Self {
        let render_context = RenderContext::new();
        Self {
            start_time: Instant::now(),
            last_render: Instant::now(),
            last_update: Instant::now(),
            render_context,
            window: None,
            surface: None,
            simple_render: None,
            rotation_rate: [0.0, 0.0, 0.0, 0.0],
        }
    }

    fn window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    fn surface(&self) -> &RenderSurface {
        self.surface.as_ref().unwrap()
    }

    fn simple_render(&self) -> &SimpleRender {
        self.simple_render.as_ref().unwrap()
    }

    fn simple_render_mut(&mut self) -> &mut SimpleRender {
        self.simple_render.as_mut().unwrap()
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

        let size = window.inner_size();
        let surface = pollster::block_on(self.render_context.create_surface(window.clone(), size.width, size.height, PresentMode::AutoNoVsync));
        self.simple_render = Some(SimpleRender::new(self.render_context.device(), self.render_context.queue(), surface.format));
        self.window = Some(window);
        self.surface = Some(surface);

        println!("Window created at {:?}", self.start_time.elapsed());
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
                if let KeyEvent {physical_key: PhysicalKey::Code(KeyCode::ArrowLeft), state, ..} = event {
                    if state.is_pressed() {
                        self.rotation_rate[1] = -1.0;
                    } else {
                        self.rotation_rate[1] = 0.0;
                    }
                }
                if let KeyEvent {physical_key: PhysicalKey::Code(KeyCode::ArrowRight), state, ..} = event {
                    if state.is_pressed() {
                        self.rotation_rate[1] = 1.0;
                    } else {
                        self.rotation_rate[1] = 0.0;
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. }=> {
                println!("WindowEvent::CursorMoved at {:?}: {:?}", self.start_time.elapsed(), position);
                self.simple_render_mut().instances[3].rotation = position.x as f32/100.0;
            }
            WindowEvent::RedrawRequested => {
                println!("WindowEvent::RedrawRequested started at {:?}", self.start_time.elapsed());

                let surface_texture = self.surface().surface.get_current_texture().unwrap();
                let surface_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                self.simple_render().render(&self.render_context, &surface_view);
                surface_texture.present();
                self.last_render = Instant::now();

                // **************************************************************************************
                // Important for exposing issues: Make render take a bit so that key repeats out run it.
                // ***************************************************************************************
                sleep(Duration::from_millis(10));

                println!("WindowEvent::RedrawRequested finished at {:?}", self.start_time.elapsed());

                self.window().request_redraw();
            }
            _ => {}
        }
    }
    
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        println!("***");
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

                if key.physical_key == PhysicalKey::Code(KeyCode::ArrowLeft) {
                    if key.state == winit::event::ElementState::Pressed {
                        self.rotation_rate[0] = -1.0;
                    } else {
                        self.rotation_rate[0] = 0.0;
                    }
                }
                if key.physical_key == PhysicalKey::Code(KeyCode::ArrowRight) {
                    if key.state == winit::event::ElementState::Pressed {
                        self.rotation_rate[0] = 1.0;
                    } else {
                        self.rotation_rate[0] = 0.0;
                    }
                }
            }
            winit::event::DeviceEvent::MouseMotion { delta } => {
                self.simple_render_mut().instances[2].rotation += delta.0 as f32/100.0;
                println!("DeviceEvent::MouseMotion at {:?}: {:?}", self.start_time.elapsed(), delta);
            }
            _ => {}
        }
    }
    
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let dt = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();

        for i in 0..4 {
            self.simple_render_mut().instances[i].rotation += self.rotation_rate[i] * dt;
        }
        // keep in range
        for i in 0..4 {
            if self.simple_render().instances[i].rotation > TAU {
                self.simple_render_mut().instances[i].rotation -= TAU;
            }
            if self.simple_render().instances[i].rotation < 0.0 {
                self.simple_render_mut().instances[i].rotation += TAU;
            }
        }
        println!("...");
    }
}

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut TestApp::new())
}

