use crate::app_state::AppState;
use crate::fv_render_callback::FvRenderCallback;
use wgpu::{Queue, RenderPass, RenderPipeline};

pub struct FvRenderer {
    pub state: AppState,
}

impl FvRenderer {
    pub fn prepare(&mut self, queue: &Queue, callback: &FvRenderCallback) {
        // if let Some(data) = &callback.shader_recompilation_options {
        //     self.pipeline = self.state.generate_pipeline(data);
        // }
        queue.write_buffer(
            &self.state.uniform_buffer,
            0,
            bytemuck::cast_slice(&[callback.uniforms]),
        );
    }

    pub fn paint(&self, render_pass: &mut RenderPass<'static>) {
        render_pass.set_pipeline(&self.state.pipeline);
        render_pass.set_bind_group(0, &self.state.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
