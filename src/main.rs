use egui::ViewportBuilder;
use josaa_orcr::AppState;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_min_inner_size([400.0, 300.0]),

        ..Default::default()
    };

    eframe::run_native(
        "JoSAA OR-CR",
        native_options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    )
}
