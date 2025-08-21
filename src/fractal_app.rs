use crate::app_state::AppState;
use crate::fv_render_callback::FvRenderCallback;
use crate::fv_renderer::FvRenderer;
use crate::uniforms::{FractalColorScheme, Uniforms};
use crate::user_settings::UserSettings;
use eframe::{CreationContext, Frame};
use egui::{ComboBox, Context, DragValue, PointerButton, Slider, Ui, Widget};

pub struct FractalApp {
    settings: UserSettings,
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
        Self {
            settings: user_settings,
        }
    }
}
impl eframe::App for FractalApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.paint_fractal(ui, ctx, frame));

        egui::Window::new("Настройки")
            // .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
            .movable(true)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Количество итераций");
                    DragValue::new(&mut self.settings.max_iter)
                        .speed(1)
                        .range(0..=255)
                        .ui(ui);
                });

                ui.separator();

                ui.heading("Центр");
                DragValue::new(&mut self.settings.center_x)
                    .speed(0.1)
                    .ui(ui);
                DragValue::new(&mut self.settings.center_y)
                    .speed(0.1)
                    .ui(ui);

                if ui.button("Сбросить").clicked() {
                    self.settings.center_x = -0.33;
                    self.settings.center_y = 0.0;
                }

                ui.separator();

                ui.heading("Масштаб");
                Slider::new(&mut self.settings.zoom, 0.2..=100_000.0).ui(ui);

                ui.separator();

                ui.heading("Цветовая схема");
                ComboBox::from_label("Цветовая схема")
                    .selected_text(format!("{}", self.settings.color_scheme))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.settings.color_scheme,
                            FractalColorScheme::RGB,
                            "RGB",
                        );
                        ui.selectable_value(
                            &mut self.settings.color_scheme,
                            FractalColorScheme::HSV,
                            "HSV",
                        );
                    });

                if self.settings.color_scheme.contains(FractalColorScheme::RGB) {
                    egui::Grid::new("rgb_settings")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Зеленый");
                            Slider::new(&mut self.settings.rgb_green, 0.0..=1.0).ui(ui);

                            ui.end_row();

                            ui.label("Голубой");
                            Slider::new(&mut self.settings.rgb_blue, 0.0..=1.0).ui(ui);
                            ui.end_row();
                        });
                }

                if self.settings.color_scheme.contains(FractalColorScheme::HSV) {
                    egui::Grid::new("hsv_settings")
                        .striped(true)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Насыщенность");
                            Slider::new(&mut self.settings.hsv_saturation, 0.0..=1.0).ui(ui);

                            ui.end_row();

                            ui.label("Яркость");
                            Slider::new(&mut self.settings.hsv_brightness, 0.0..=1.0).ui(ui);

                            ui.end_row();
                        });
                }
            });
    }
}

impl FractalApp {
    fn paint_fractal(&mut self, ui: &mut Ui, _ctx: &Context, _frame: &mut Frame) {
        ui.label("Hello, world!");

        let size = ui.available_size().max(egui::vec2(400.0, 400.0));
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

        let scale = 4.0 / self.settings.zoom / size.min_elem();
        if response.dragged_by(PointerButton::Primary) {
            let drag_motion = response.drag_delta();
            self.settings.center_x -= drag_motion.x * scale;
            self.settings.center_y += drag_motion.y * scale;
        }

        let scroll = ui.input(|i| i.raw_scroll_delta);

        self.settings.zoom += self.settings.zoom * (scroll.y / 300.0).max(-0.9);
        let user_settings = &self.settings;
        let uniforms = Uniforms {
            max_iter: user_settings.max_iter,
            zoom: user_settings.zoom,
            center: [user_settings.center_x, user_settings.center_y, 0.0, 0.0],
            rgb_green: user_settings.rgb_green,
            rgb_blue: user_settings.rgb_blue,
            color_scheme: self.settings.color_scheme.bits(),
            hsv_saturation: self.settings.hsv_saturation,
            hsv_brightness: self.settings.hsv_brightness,
            _pad1: [0; 4],
        };
        let callback = FvRenderCallback { uniforms };

        ui.painter()
            .add(egui_wgpu::Callback::new_paint_callback(rect, callback));
    }
}
