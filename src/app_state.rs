pub struct AppState {}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {}
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("Hello world!");
            });
        });
    }
}
