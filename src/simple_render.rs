use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, Device, Queue, RenderPipeline, TextureFormat};

use crate::render_context::RenderContext;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TriInstance {
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: f32,
}

pub struct SimpleRender {
    pub render_pipeline: RenderPipeline,
    pub instance_buffer: Buffer,
    pub instances: Vec<TriInstance>,
}

impl SimpleRender {
    pub fn new(device: &Device, queue: &Queue, surface_format: TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let mut instances = Vec::new();
        instances.push(TriInstance {
            position: [-0.35, 0.35],
            rotation: 0.0,
            scale: 1.0,
        });
        instances.push(TriInstance {
            position: [0.35, 0.35],
            rotation: 0.0,
            scale: 1.0,
        });
        instances.push(TriInstance {
            position: [-0.35, -0.35],
            rotation: 0.0,
            scale: 1.0,
        });
        instances.push(TriInstance {
            position: [0.35, -0.35],
            rotation: 0.0,
            scale: 1.0,
        });

        let instance_buffer_desc = wgpu::BufferDescriptor {
            label: None,
            size: instances.len() as u64 * std::mem::size_of::<TriInstance>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };

        let instance_buffer = device.create_buffer(&instance_buffer_desc);
        queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(&instances[..]));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    // instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<TriInstance>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // position
                            wgpu::VertexAttribute {
                                offset: 0,
                                format: wgpu::VertexFormat::Float32x2,
                                shader_location: 0,
                            },
                            // rotation
                            wgpu::VertexAttribute {
                                offset: 8,
                                format: wgpu::VertexFormat::Float32,
                                shader_location: 1,
                            },
                            // scale
                            wgpu::VertexAttribute {
                                offset: 12,
                                format: wgpu::VertexFormat::Float32,
                                shader_location: 2,
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: Default::default(),
        });

        Self { render_pipeline, instance_buffer, instances }
    }

    pub fn render(&self, render_context: &RenderContext, texture_view: &wgpu::TextureView) {

        render_context.queue().write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances[..]));

        let mut encoder = render_context.device().create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: texture_view,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 0.0,
                }),
                store: wgpu::StoreOp::Store,
            },
            resolve_target: None,
        };
    
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            let instance_count = self.instances.len() as u32;

            render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..instance_count);
        }

        let encoded = encoder.finish();

        render_context.queue().submit(Some(encoded));
    }
}


const SHADER: &str = r#"

struct InstanceInput {
    @location(0) position: vec2<f32>,
    @location(1) rotation: f32,
    @location(2) scale: f32,
};

//-------------------------------------------------
// Vertex shader
//-------------------------------------------------

@vertex
fn vs_main(@builtin(vertex_index) idx: u32, instance: InstanceInput) -> @builtin(position) vec4<f32> {
    let x_local = instance.scale * 0.1 * f32(i32(idx) - 1);
    let y_local = instance.scale * 0.15 * f32(i32(idx & 1u) * 2 - 1);

    var x =  x_local*cos(instance.rotation) + y_local*sin(instance.rotation) + instance.position.x;
    var y = -x_local*sin(instance.rotation) + y_local*cos(instance.rotation) + instance.position.y;

    return vec4<f32>(x, y, 0.0, 1.0);
}

//-------------------------------------------------
// Fragment shader
//-------------------------------------------------

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
"#;