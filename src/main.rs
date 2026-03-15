use eframe::NativeOptions;
use egui::ViewportBuilder;
use egui_wgpu::WgpuConfiguration;
use mandelbrot_gpu::fractal_app::FractalApp;
use mimalloc::MiMalloc;
use wgpu::PresentMode;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let width = 1600.0;
    let height = width / 1.5;
    let options = NativeOptions {
        centered: true,
        viewport: ViewportBuilder::default()
            .with_title("Mandelbrot WGPU")
            .with_inner_size([width, height])
            .with_always_on_top(),
        wgpu_options: WgpuConfiguration {
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Mandelbrot wgpu",
        options,
        Box::new(|cc| Ok(Box::new(FractalApp::new(cc)))),
    )?;

    Ok(())
}
