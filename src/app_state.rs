use bytemuck::{Pod, Zeroable};
use wgpu::wgt::BufferDescriptor;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device,
    FragmentState, PipelineLayoutDescriptor, RenderPipeline, RenderPipelineDescriptor,
    ShaderStages, TextureFormat, VertexState, include_wgsl,
};

pub struct AppState {
    pub bind_group: BindGroup,
    pub pipeline: RenderPipeline,
    pub uniform_buffer: Buffer,
    pub format: TextureFormat,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub limit: u32,
    pub _padding: [u8; 4],
    pub center: [f32; 2],
}

impl AppState {
    pub async fn new(device: &Device, format: TextureFormat) -> Result<Self, anyhow::Error> {
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Params buffer"),
            size: size_of::<Uniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("main bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("main bind group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let module = device.create_shader_module(include_wgsl!("mandelbrot.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("main pipeline descriptor"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("main render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            multiview: None,
            cache: None,
        });
        Ok(Self {
            format,
            bind_group,
            uniform_buffer,
            pipeline,
        })
    }
}
