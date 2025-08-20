use crate::app_state::{AppState, Uniforms};
use egui::PaintCallbackInfo;
use egui_wgpu::wgpu::RenderPass;
use egui_wgpu::{CallbackResources, CallbackTrait, Renderer};
use std::sync::Arc;
use wgpu::wgt::CommandEncoderDescriptor;
use wgpu::{
    Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    StoreOp,
};

// #[derive(Default)]
// pub struct App {
// state: Option<AppState<'a>>,
// width: u32,
// height: u32,
// }

// let state =
//     pollster::block_on(AppState::new(window.clone())).expect("Failed to create state");
// self.window = Some(window);
// self.state = Some(state);
// }
//
// fn window_event(
//     &mut self,
//     event_loop: &ActiveEventLoop,
//     _window_id: WindowId,
//     event: WindowEvent,
// ) {
//     let state = self.state.as_mut().unwrap();
// match event {
//     WindowEvent::ActivationTokenDone { .. } => {}
//     WindowEvent::Resized(s) => {
//         let options = &mut state.surface_options;
//         options.width = s.width;
//         options.height = s.height;
//         state.surface.configure(&state.device, &options)
//     }
//     WindowEvent::Moved(_) => {}
//     WindowEvent::CloseRequested => event_loop.exit(),
//     WindowEvent::Destroyed => {}
//     WindowEvent::DroppedFile(_) => {}
//     WindowEvent::HoveredFile(_) => {}
//     WindowEvent::HoveredFileCancelled => {}
//     WindowEvent::Focused(_) => {}
//     WindowEvent::KeyboardInput { .. } => {}
//     WindowEvent::ModifiersChanged(_) => {}
//     WindowEvent::Ime(_) => {}
//     WindowEvent::CursorMoved { .. } => {}
//     WindowEvent::CursorEntered { .. } => {}
//     WindowEvent::CursorLeft { .. } => {}
//     WindowEvent::MouseWheel { .. } => {}
//     WindowEvent::MouseInput { .. } => {}
//     WindowEvent::PinchGesture { .. } => {}
//     WindowEvent::PanGesture { .. } => {}
//     WindowEvent::DoubleTapGesture { .. } => {}
//     WindowEvent::RotationGesture { .. } => {}
//     WindowEvent::TouchpadPressure { .. } => {}
//     WindowEvent::AxisMotion { .. } => {}
//     WindowEvent::Touch(_) => {}
//     WindowEvent::ScaleFactorChanged { .. } => {}
//     WindowEvent::ThemeChanged(_) => {}
//     WindowEvent::Occluded(_) => {}
//     WindowEvent::RedrawRequested => {
//         let frame = state
//             .surface
//             .get_current_texture()
//             .expect("Failed to get surface texture");
//
//         let view = frame.texture.create_view(&Default::default());
//         let mut encoder = state
//             .device
//             .create_command_encoder(&CommandEncoderDescriptor {
//                 label: Some("Main command encoder"),
//             });
//
//         let params = Uniforms {
//             limit: 255,
//             _padding: [0; 4],
//             center: [0.0, 0.0],
//         };
//         state
//             .queue
//             .write_buffer(&state.params_buffer, 0, bytemuck::bytes_of(&params));
//         {
//             let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
//                 label: Some("ma"),
//                 color_attachments: &[Some(RenderPassColorAttachment {
//                     view: &view,
//                     depth_slice: None,
//                     resolve_target: None,
//                     ops: Operations {
//                         load: LoadOp::Clear(Color {
//                             r: 0.0,
//                             g: 0.0,
//                             b: 0.0,
//                             a: 1.0,
//                         }),
//                         store: StoreOp::Store,
//                     },
//                 })],
//                 depth_stencil_attachment: None,
//                 timestamp_writes: None,
//                 occlusion_query_set: None,
//             });
//
//             render_pass.set_pipeline(&state.pipeline);
//             render_pass.set_bind_group(0, &state.bind_group, &[]);
//             render_pass.draw(0..6, 0..1);
//         }
//
//         state.queue.submit(Some(encoder.finish()));
//
//         frame.present();
//     }
// }
// }
// }
