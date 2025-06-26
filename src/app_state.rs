use crate::{
    Dataset, Entry, Options, Sort,
    widgets::{Dropdown, Multiselect, RangeSelector},
};
use egui_extras::{Column, TableBuilder};

pub struct AppState {
    dataset: Dataset,
    options: Options,
    sort: Sort,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            dataset: Dataset::new(),
            options: Options::default(),
            sort: Sort::default(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Header
        egui::TopBottomPanel::top("Top Panel")
            .resizable(true)
            .show(ctx, |ui| {
                // Dataset selection
                egui::CollapsingHeader::new("Choose a dataset")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Year selection
                            Dropdown::with_state(&mut self.options.year)
                                .with_label("Year")
                                .with_options(Options::get_valid_years().map(Some))
                                .show(ui, |state| match state {
                                    Some(selected) => selected.to_string(),
                                    None => "Select".into(),
                                });

                            // Round selection
                            Dropdown::with_state(&mut self.options.round)
                                .with_label("Round")
                                .with_options(
                                    Options::get_valid_rounds(self.options.year).map(Some),
                                )
                                .with_enabled(self.options.year.is_some())
                                .show(ui, |state| match state {
                                    Some(selected) => selected.to_string(),
                                    None => "Select".into(),
                                });

                            // Load button
                            if ui
                                .add_enabled(
                                    self.options.is_complete(),
                                    egui::Button::new("Load dataset"),
                                )
                                .clicked()
                            {
                                self.dataset.load(&self.options).unwrap();
                                self.dataset.sort(&self.sort);
                            }
                        });
                    });

                // Institute filter
                ui.add_enabled_ui(self.dataset.is_loaded(), |ui| {
                    ui.collapsing("Institute", |ui| {
                        let filters = self.dataset.get_filters();

                        for (label, (enabled, institutes)) in filters.institute_kinds.iter_mut() {
                            egui::collapsing_header::CollapsingState::load_with_default_open(
                                ui.ctx(),
                                ui.make_persistent_id(format!("kind_{label}")),
                                false,
                            )
                            .show_header(ui, |ui| {
                                ui.checkbox(enabled, label);
                            })
                            .body_unindented(|ui| {
                                ui.horizontal_wrapped(|ui| {
                                    for (value, checked) in institutes.iter_mut() {
                                        ui.checkbox(checked, value).clicked();
                                    }
                                });
                            });
                        }
                    });
                });

                // Filters
                ui.add_enabled_ui(self.dataset.is_loaded(), |ui| {
                    let filters = self.dataset.get_filters();

                    ui.collapsing("Filters", |ui| {
                        // Quota
                        Multiselect::with_state(&mut filters.quota)
                            .with_label("Quota")
                            .show(ui);

                        // Seat type
                        Multiselect::with_state(&mut filters.seat_type)
                            .with_label("Seat type")
                            .show(ui);

                        // Gender
                        Multiselect::with_state(&mut filters.gender)
                            .with_label("Gender")
                            .show(ui);

                        // Opening rank
                        RangeSelector::with_state(&mut filters.or, &filters.or_bounds)
                            .with_label("Opening rank")
                            .show(ui);

                        // Closing rank
                        RangeSelector::with_state(&mut filters.cr, &filters.cr_bounds)
                            .with_label("Closing rank")
                            .show(ui);
                    });
                });

                // Sort selection
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(self.dataset.is_loaded(), |ui| {
                        match Dropdown::with_state(&mut self.sort)
                            .with_label("Sort by")
                            .with_options(Sort::as_vec().into_iter())
                            .show(ui, Sort::to_string)
                        {
                            Some(response) if response.changed() => {
                                self.dataset.sort(&self.sort);
                            }
                            _ => (),
                        }
                    })
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
                        let entries: Vec<&Entry> = self.dataset.get_entries().collect();

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
