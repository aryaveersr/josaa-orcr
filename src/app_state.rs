use crate::{Dataset, Options, widgets::Dropdown};
use egui_extras::{Column, TableBuilder};

pub struct AppState {
    dataset: Dataset,
    options: Options,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            dataset: Dataset::new(),
            options: Options::default(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Header
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            // Dataset selection
            egui::CollapsingHeader::new("Choose a dataset")
                .default_open(true)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Year selection
                        Dropdown::with_state(&mut self.options.year)
                            .with_label("Year")
                            .with_options(Options::get_valid_years())
                            .show(ui);

                        // Round selection
                        Dropdown::with_state(&mut self.options.round)
                            .with_label("Round")
                            .with_options(Options::get_valid_rounds(self.options.year))
                            .with_enabled(self.options.year.is_some())
                            .show(ui);

                        // Load button
                        if ui
                            .add_enabled(
                                self.options.is_complete(),
                                egui::Button::new("Load dataset"),
                            )
                            .clicked()
                        {
                            self.dataset.load(&self.options).unwrap();
                        }
                    });
                });
        });

        // Center Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Table view of dataset
            if self.dataset.is_loaded() {
                TableBuilder::new(ui)
                    .column(Column::remainder())
                    .columns(Column::auto(), 5)
                    .header(24.0, |mut header| {
                        header.col(label("Institute"));
                        header.col(label("Quota"));
                        header.col(label("Seat type"));
                        header.col(label("Gender"));
                        header.col(label("Opening Rank"));
                        header.col(label("Closing Rank"));
                    })
                    .body(|body| {
                        let entries = self.dataset.get_entries();

                        body.rows(30.0, entries.len(), |mut row| {
                            let data = &entries[row.index()];

                            row.col(label(&data.institute));
                            row.col(label(&data.quota));
                            row.col(label(&data.seat_type));
                            row.col(label(&data.gender));
                            row.col(label(data.or));
                            row.col(label(data.cr));
                        });
                    });
            }
            // Empty state
            else {
                ui.centered_and_justified(|ui| {
                    ui.label("No dataset selected.");
                });
            }
        });
    }
}

fn label<T: ToString>(content: T) -> impl FnOnce(&mut egui::Ui) {
    move |ui| {
        ui.label(content.to_string());
    }
}
