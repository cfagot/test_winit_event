use wgpu::{Adapter, Device, Instance, Limits, Queue, Surface, SurfaceConfiguration, SurfaceTarget, TextureFormat};

pub struct RenderContext {
    pub instance: Instance,
    pub adapter: Option<Adapter>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
}

pub struct RenderSurface<'w> {
    pub surface: Surface<'w>,
    pub config: SurfaceConfiguration,
    pub format: TextureFormat,
}

impl RenderContext {
    pub fn new() -> Self {
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            ..Default::default()
        });
        Self {
            instance,
            adapter: None,
            device: None,
            queue: None,
        }
    }

    /// Creates a new surface for the specified window and dimensions.
    pub async fn create_surface<'w>(
        &mut self,
        window: impl Into<SurfaceTarget<'w>>,
        width: u32,
        height: u32,
        present_mode: wgpu::PresentMode,
    ) -> RenderSurface<'w> {
        let surface = self.instance.create_surface(window.into()).unwrap();

        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&self.instance, Some(&surface)).await.unwrap();
        let features = adapter.features();
        let limits = Limits::default();
        let mut maybe_features = wgpu::Features::CLEAR_TEXTURE;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: features & maybe_features,
                    required_limits: limits,
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();

        self.device = Some(device);
        self.queue = Some(queue);
        self.adapter = Some(adapter);

        let device = self.device.as_ref().unwrap();
        let adapter = self.adapter.as_ref().unwrap();

        let capabilities = surface.get_capabilities(adapter);
        let format = *capabilities.formats.first().unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        let surface = RenderSurface {
            surface,
            config,
            format,
        };
        surface.surface.configure(self.device.as_ref().unwrap(), &surface.config);
        return surface;
    }

    pub fn set_present_mode(&self, surface: &mut RenderSurface, present_mode: wgpu::PresentMode) {
        surface.config.present_mode = present_mode;
        surface.surface.configure(self.device.as_ref().unwrap(), &surface.config);
    }

}