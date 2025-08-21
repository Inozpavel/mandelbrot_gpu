use eframe::NativeOptions;
use egui::ViewportBuilder;
use mandelbrot_gpu::fractal_app::FractalApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let width = 1200.0;
    let height = width / 1.5;
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Mandelbrot WGPU")
            .with_inner_size([width, height]),
        vsync: true,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Mandelbrot wgpu",
        options,
        Box::new(|cc| Ok(Box::new(FractalApp::new(cc)))),
    )?;

    Ok(())
}
