use crate::fv_renderer::FvRenderer;
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
        let renderer: &mut FvRenderer = callback_resources.get_mut().unwrap();
        renderer.prepare(queue, self);
        vec![]
    }

    fn paint(
        &self,
        info: PaintCallbackInfo,
        render_pass: &mut RenderPass<'static>,
        callback_resources: &CallbackResources,
    ) {
        let renderer: &FvRenderer = callback_resources.get().expect("Missing FvRenderer");
        renderer.paint(render_pass);
    }
}
