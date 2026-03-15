use crate::fv_renderer_resource::FvRendererResource;
use crate::uniforms::Uniforms;
use eframe::epaint::PaintCallbackInfo;
use egui_wgpu::wgpu::RenderPass;
use egui_wgpu::{CallbackResources, CallbackTrait, ScreenDescriptor};
use wgpu::{CommandBuffer, CommandEncoder, Device, Queue};

pub struct FvRenderCallback {
    pub uniforms: Uniforms,
}

impl CallbackTrait for FvRenderCallback {
    fn prepare(
        &self,
        _device: &Device,
        queue: &Queue,
        _screen_descriptor: &ScreenDescriptor,
        _egui_encoder: &mut CommandEncoder,
        callback_resources: &mut CallbackResources,
    ) -> Vec<CommandBuffer> {
        let resource = callback_resources
            .get::<FvRendererResource>()
            .expect("Missing FvRendererResource");

        queue.write_buffer(
            &resource.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
        vec![]
    }

    fn paint(
        &self,
        _info: PaintCallbackInfo,
        render_pass: &mut RenderPass<'static>,
        callback_resources: &CallbackResources,
    ) {
        let resource = callback_resources
            .get::<FvRendererResource>()
            .expect("Missing FvRendererResource");

        render_pass.set_pipeline(&resource.pipeline);
        render_pass.set_bind_group(0, &resource.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
