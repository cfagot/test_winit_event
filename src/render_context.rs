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

    pub fn queue(&self) -> &Queue {
        self.queue.as_ref().unwrap()
    }

    pub fn device(&self) -> &Device {
        self.device.as_ref().unwrap()
    }

    pub fn adapter(&self) -> &Adapter {
        self.adapter.as_ref().unwrap()
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
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: features,
                    required_limits: limits,
                },
                None,
            )
            .await
            .unwrap();

        self.device = Some(device);
        self.queue = Some(queue);
        self.adapter = Some(adapter);

        let adapter = self.adapter();

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
        surface.surface.configure(self.device(), &surface.config);
        return surface;
    }
}