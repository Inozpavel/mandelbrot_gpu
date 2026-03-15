use crate::uniforms::Uniforms;
use egui_wgpu::RenderState;
use wgpu::wgt::BufferDescriptor;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, FragmentState,
    PipelineLayoutDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderStages, VertexState,
    include_wgsl,
};

pub struct FvRendererResource {
    pub bind_group: BindGroup,
    pub pipeline: RenderPipeline,
    pub uniform_buffer: Buffer,
}

impl FvRendererResource {
    pub fn new(render_state: &RenderState) -> Self {
        let device = &render_state.device;
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
                targets: &[Some(render_state.target_format.into())],
            }),
            multiview: None,
            cache: None,
        });
        Self {
            bind_group,
            uniform_buffer,
            pipeline,
        }
    }
}
