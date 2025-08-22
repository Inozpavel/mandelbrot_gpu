use crate::app_state::AppState;
use crate::fv_render_callback::FvRenderCallback;
use crate::fv_renderer::FvRenderer;
use crate::uniforms::{FractalColorScheme, FractalType, Uniforms};
use crate::user_settings::UserSettings;
use eframe::{CreationContext, Frame};
use egui::{Context, DragValue, Grid, Key, PointerButton, Slider, Ui, ViewportCommand, Widget};
use log::info;
use measure_time::debug_time;
use std::time::Instant;

const MAX_FRAMES_SAMPLES: usize = 200;
pub struct FractalApp {
    settings: UserSettings,
    adapter_name: String,
    backend_name: String,
    driver_version: String,
    driver_name: String,
    last_frame_time: Instant,
    fps_samples: [f32; MAX_FRAMES_SAMPLES],
    current_index: usize,
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

        let adapter_info = wgpu_render_state.adapter.get_info();
        let backend_name = format!("{}", adapter_info.backend);
        let user_settings = UserSettings::new();
        info!("{:?}", &adapter_info);
        Self {
            settings: user_settings,
            adapter_name: adapter_info.name.clone(),
            backend_name,
            driver_version: adapter_info.driver_info.clone(),
            driver_name: adapter_info.driver.clone(),
            last_frame_time: Instant::now(),
            fps_samples: [0.0; 200],
            current_index: 0,
        }
    }
}

impl eframe::App for FractalApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        debug_time!("Frame logic");
        if ctx.input(|state| state.key_pressed(Key::F1)) {
            self.settings.show_settings = !self.settings.show_settings;
        }
        if ctx.input(|state| state.key_pressed(Key::F11))
            && let Some(current_is_fullscreen) = ctx.input(|i| i.viewport().fullscreen)
        {
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(!current_is_fullscreen))
        }
        egui::CentralPanel::default().show(ctx, |ui| self.paint_fractal(ui, ctx, frame));

        egui::Window::new("Информация и настройки")
            .open(&mut self.settings.show_settings)
            .movable(true)
            .default_pos([0.0, 0.0])
            .resizable(false)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    Grid::new("fractal")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.heading("Версия приложения");
                            ui.label(env!("CARGO_PKG_VERSION").to_string());

                            ui.end_row();

                            ui.heading("ОС & архитектура");
                            ui.label(format!(
                                "{} {}",
                                std::env::consts::OS,
                                std::env::consts::ARCH
                            ));
                            ui.end_row();

                            ui.heading("Адаптер");
                            ui.label(self.adapter_name.to_string());

                            ui.end_row();

                            ui.heading("Backend");
                            ui.label(format!(
                                "{} ({} {})",
                                self.backend_name, self.driver_name, self.driver_version
                            ));

                            ui.end_row();

                            let last_frame_elapsed = self.last_frame_time.elapsed();

                            self.fps_samples[self.current_index] =
                                last_frame_elapsed.as_secs_f32().recip();
                            self.current_index += 1;

                            if self.current_index >= MAX_FRAMES_SAMPLES {
                                self.current_index = 0;
                            }

                            ui.heading("Среднее количество кадров в секунду");

                            let avg = self.fps_samples.iter().sum::<f32>()
                                / self.fps_samples.len() as f32;

                            ui.label(format!("{avg:.0}"));
                            ui.end_row();

                            ui.heading("Время отрисовки последнего кадра");
                            ui.label(format!("{last_frame_elapsed:?}"));
                            ui.end_row();
                        });
                });

                ui.group(|ui| {
                    Grid::new("settings")
                        .num_columns(2)
                        .striped(true)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.heading("Тип фрактала");

                            ui.horizontal(|ui| {
                                ui.selectable_value(
                                    &mut self.settings.fractal_type,
                                    FractalType::MANDELBROT,
                                    FractalType::MANDELBROT.to_string(),
                                );
                                ui.selectable_value(
                                    &mut self.settings.fractal_type,
                                    FractalType::JULIA,
                                    FractalType::JULIA.to_string(),
                                );
                            });

                            ui.end_row();

                            ui.heading("Количество итераций");
                            DragValue::new(&mut self.settings.max_iter)
                                .speed(1)
                                .range(0..=255)
                                .ui(ui);

                            ui.end_row();

                            ui.heading("Граница проверки");
                            DragValue::new(&mut self.settings.escape_threshold)
                                .speed(1.0)
                                .range(1.0..=255.0)
                                .ui(ui);

                            ui.end_row();

                            ui.heading("Центр");
                            ui.horizontal(|ui| {
                                DragValue::new(&mut self.settings.center_x)
                                    .speed(0.1)
                                    .ui(ui);

                                DragValue::new(&mut self.settings.center_y)
                                    .speed(0.1)
                                    .suffix("i")
                                    .ui(ui);

                                if ui.button("Сбросить").clicked() {
                                    self.settings.center_x = -0.33;
                                    self.settings.center_y = 0.0;
                                }
                            });

                            ui.end_row();

                            ui.heading("Масштаб");
                            ui.horizontal(|ui| {
                                Slider::new(&mut self.settings.zoom, 0.2..=100_000.0).ui(ui);
                                if ui.button("Сбросить").clicked() {
                                    self.settings.zoom = 1.0;
                                }
                            });

                            ui.end_row();

                            ui.heading("Цветовая схема");

                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Текущая");

                                    ui.horizontal(|ui| {
                                        ui.selectable_value(
                                            &mut self.settings.color_scheme,
                                            FractalColorScheme::RGB,
                                            FractalColorScheme::RGB.to_string(),
                                        );

                                        ui.selectable_value(
                                            &mut self.settings.color_scheme,
                                            FractalColorScheme::HSV,
                                            FractalColorScheme::HSV.to_string(),
                                        );
                                    });
                                });

                                if self.settings.color_scheme.contains(FractalColorScheme::RGB) {
                                    Grid::new("rgb_settings")
                                        .num_columns(2)
                                        .spacing([10.0, 4.0])
                                        .show(ui, |ui| {
                                            ui.label("Зеленый");
                                            Slider::new(&mut self.settings.rgb_green, 0.0..=1.0)
                                                .ui(ui);

                                            ui.end_row();

                                            ui.label("Голубой");
                                            Slider::new(&mut self.settings.rgb_blue, 0.0..=1.0)
                                                .ui(ui);
                                            ui.end_row();
                                        });
                                }

                                if self.settings.color_scheme.contains(FractalColorScheme::HSV) {
                                    Grid::new("hsv_settings")
                                        .num_columns(2)
                                        .spacing([10.0, 4.0])
                                        .show(ui, |ui| {
                                            ui.label("Насыщенность");
                                            Slider::new(
                                                &mut self.settings.hsv_saturation,
                                                0.0..=1.0,
                                            )
                                            .ui(ui);

                                            ui.end_row();

                                            ui.label("Яркость");
                                            Slider::new(
                                                &mut self.settings.hsv_brightness,
                                                0.0..=1.0,
                                            )
                                            .ui(ui);

                                            ui.end_row();
                                        });
                                }
                            });

                            ui.end_row();

                            ui.heading("Начальное значение");

                            ui.horizontal(|ui| {
                                DragValue::new(&mut self.settings.initial_value_x)
                                    .speed(0.01)
                                    .range(-3.0..=3.0)
                                    .ui(ui);
                                DragValue::new(&mut self.settings.initial_value_y)
                                    .speed(0.01)
                                    .range(-3.0..=3.0)
                                    .suffix("i")
                                    .ui(ui);
                                if ui.button("Сбросить").clicked() {
                                    self.settings.initial_value_x = 0.0;
                                    self.settings.initial_value_y = 0.0;
                                }
                            });
                            ui.end_row();

                            ui.heading("Показывать оси");
                            ui.checkbox(&mut self.settings.show_axis, "");
                            ui.end_row();
                        });
                });

                ui.group(|ui| {
                    ui.label("F1 - скрыть настройки");
                    ui.label("F11 - полноэкранный режим");
                    ui.label("Колесо мыши - изменить масштаб");
                    ui.label("ЛКМ + движение мыши - изменить координаты");
                    ui.label("ПКМ + движение мыши - изменить начальное значение");
                });
            });

        self.last_frame_time = Instant::now();
    }
}

impl FractalApp {
    fn paint_fractal(&mut self, ui: &mut Ui, _ctx: &Context, _frame: &mut Frame) {
        let size = ui.available_size().max(egui::vec2(400.0, 400.0));
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

        let scale = 4.0 / self.settings.zoom / size.min_elem();
        if response.dragged_by(PointerButton::Primary) {
            let drag_motion = response.drag_delta();
            self.settings.center_x -= drag_motion.x * scale;
            self.settings.center_y += drag_motion.y * scale;
        }

        if response.dragged_by(PointerButton::Secondary) {
            let drag_motion = response.drag_delta();
            self.settings.initial_value_x -= drag_motion.x * scale / 2.0;
            self.settings.initial_value_y += drag_motion.y * scale / 2.0;
        }

        let scroll = ui.input(|i| i.raw_scroll_delta);

        self.settings.zoom += self.settings.zoom * (scroll.y / 380.0).max(-0.9);
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
            show_axis: self.settings.show_axis as u8 as u32,
            escape_threshold: self.settings.escape_threshold,
            initial_value: [
                self.settings.initial_value_x,
                self.settings.initial_value_y,
                0.0,
                0.0,
            ],
            fractal_type: self.settings.fractal_type.bits(),
            pad: [0; 8],
        };
        let callback = FvRenderCallback { uniforms };

        ui.painter()
            .add(egui_wgpu::Callback::new_paint_callback(rect, callback));
    }
}
