use egui::ViewportBuilder;
use josaa_orcr::AppState;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size([800.0, 600.0])
            .with_maximized(true),

        ..Default::default()
    };

    eframe::run_native(
        "JoSAA OR-CR",
        native_options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    )
}
