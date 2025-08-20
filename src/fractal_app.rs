use crate::app_state::{AppState, Uniforms};
use crate::fv_render_callback::FvRenderCallback;
use crate::fv_renderer::FvRenderer;
use crate::user_settings::UserSettings;
use eframe::{CreationContext, Frame};
use egui::{Context, DragValue, Ui, Widget};

pub struct FractalApp {
    user_settings: UserSettings,
}

impl FractalApp {
    pub fn new(cc: &CreationContext) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().expect("no wgpu_render_state");
        let device = &wgpu_render_state.device;

        let app_state = pollster::block_on(AppState::new(device, wgpu_render_state.target_format))
            .expect("Failed to create app state");
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(FvRenderer { state: app_state });

        let user_settings = UserSettings::new();
        Self { user_settings }
    }
}
impl eframe::App for FractalApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.paint_fractal(ui, ctx, frame));

        egui::Window::new("Настройки")
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0])
            .movable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Количество итераций");
                    DragValue::new(&mut self.user_settings.iterations_limit)
                        .speed(1)
                        .range(0..=255)
                        .ui(ui);
                });

                ui.heading("Центр");
                DragValue::new(&mut self.user_settings.center_x)
                    .speed(0.1)
                    .ui(ui);
                DragValue::new(&mut self.user_settings.center_y)
                    .speed(0.1)
                    .ui(ui);
            });
    }
}

impl FractalApp {
    fn paint_fractal(&mut self, ui: &mut Ui, ctx: &Context, frame: &mut Frame) {
        ui.label("Hello, world!");

        let size = ui.available_size().max(egui::vec2(400.0, 400.0));
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

        let user_settings = &self.user_settings;
        let uniforms = Uniforms {
            limit: user_settings.iterations_limit,
            _padding: [0; 4],
            center: [user_settings.center_x, user_settings.center_y],
        };
        let callback = FvRenderCallback { uniforms };

        ui.painter()
            .add(egui_wgpu::Callback::new_paint_callback(rect, callback));
    }
}
