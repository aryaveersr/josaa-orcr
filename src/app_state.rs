use crate::{Dataset, DatasetOptions, VALID_YEARS, valid_rounds, widgets::Dropdown};
use egui_extras::{Column, TableBuilder};

pub struct AppState {
    dataset: Option<Dataset>,
    dataset_options: DatasetOptions,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            dataset: None,
            dataset_options: DatasetOptions::default(),
        }
    }

    fn load_dataset(&mut self) {
        let mut dataset = Dataset::new(&self.dataset_options).unwrap();

        dataset.fetch_rows().unwrap();
        self.dataset = Some(dataset);
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
                        Dropdown::with_state(&mut self.dataset_options.year)
                            .label("Year")
                            .show(ui, || VALID_YEARS);

                        // Round selection
                        Dropdown::with_state(&mut self.dataset_options.round)
                            .label("Round")
                            .show(ui, || valid_rounds(self.dataset_options.year.unwrap()));

                        // Load button
                        if ui
                            .add_enabled(
                                self.dataset_options.year.is_some()
                                    && self.dataset_options.round.is_some(),
                                egui::Button::new("Load dataset"),
                            )
                            .clicked()
                        {
                            self.load_dataset();
                        }
                    });
                });
        });

        // Center Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Table view of dataset
            if let Some(dataset) = &mut self.dataset {
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
                        body.rows(30.0, dataset.rows.len(), |mut row| {
                            let data = &dataset.rows[row.index()];

                            row.col(label(&data.institute));
                            row.col(label(&data.quota));
                            row.col(label(&data.seat_type));
                            row.col(label(&data.gender));
                            row.col(label(&data.or.to_string()));
                            row.col(label(&data.cr.to_string()));
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

fn label(content: &str) -> impl FnOnce(&mut egui::Ui) {
    move |ui| {
        ui.label(content);
    }
}
