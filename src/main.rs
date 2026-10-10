mod app;
mod models;
mod theme;
mod ui;

use app::LauncherApp;
use eframe::egui;

fn main() -> eframe::Result {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 780.0])
            .with_min_inner_size([1024.0, 640.0])
            .with_maximized(true)
            .with_title("StudioGrade — Video Project Hub"),
        ..Default::default()
    };

    eframe::run_native(
        "StudioGrade — Video Project Hub",
        native_options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(LauncherApp::default()))
        }),
    )
}
