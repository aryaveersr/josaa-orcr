use crate::{
    Dataset, Filters, Options, Sort,
    widgets::{Dropdown, Multiselect, RangeSelector},
};
use egui_extras::{Column, TableBuilder};

pub struct AppState {
    dataset: Dataset,
    options: Options,

    filter_values: Option<Filters>,
    filters: Option<Filters>,

    sort: Sort,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            dataset: Dataset::new(),
            options: Options::default(),

            filter_values: None,
            filters: None,

            sort: Sort::default(),
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
                            .with_options(Options::get_valid_rounds(self.options.year).map(Some))
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
                            self.dataset.create_connection(&self.options).unwrap();

                            self.filters =
                                Some(Filters::new(self.dataset.get_connection()).unwrap());
                            self.filter_values = self.filters.clone();

                            self.dataset
                                .load(self.filters.as_ref().unwrap(), self.sort)
                                .unwrap();
                        }
                    });
                });

            // Filters
            ui.add_enabled_ui(self.dataset.is_loaded(), |ui| {
                ui.collapsing("Filters", |ui| {
                    // Quota
                    Multiselect::with_state(&mut self.filters.as_mut().unwrap().quota)
                        .with_label("Quota")
                        .with_options(
                            self.filter_values
                                .as_ref()
                                .unwrap()
                                .quota
                                .clone()
                                .into_iter()
                                .map(|i| i.into()),
                        )
                        .show(ui);

                    // Seat type
                    Multiselect::with_state(&mut self.filters.as_mut().unwrap().seat_type)
                        .with_label("Seat type")
                        .with_options(
                            self.filter_values
                                .as_ref()
                                .unwrap()
                                .seat_type
                                .clone()
                                .into_iter()
                                .map(|i| i.into()),
                        )
                        .show(ui);

                    // Gender
                    Multiselect::with_state(&mut self.filters.as_mut().unwrap().gender)
                        .with_label("Gender")
                        .with_options(
                            self.filter_values
                                .as_ref()
                                .unwrap()
                                .gender
                                .clone()
                                .into_iter()
                                .map(|i| i.into()),
                        )
                        .show(ui);

                    // Opening rank
                    RangeSelector::with_state(
                        &mut self.filters.as_mut().unwrap().or,
                        &self.filter_values.as_ref().unwrap().or,
                    )
                    .with_label("Opening rank")
                    .show(ui);

                    // Closing rank
                    RangeSelector::with_state(
                        &mut self.filters.as_mut().unwrap().cr,
                        &self.filter_values.as_ref().unwrap().cr,
                    )
                    .with_label("Closing rank")
                    .show(ui);

                    // Apply filters
                    if ui.button("Apply filters").clicked() {
                        self.dataset
                            .load(self.filters.as_ref().unwrap(), self.sort)
                            .unwrap();
                    }
                });
            });

            // Sort selection
            ui.horizontal(|ui| {
                Dropdown::with_state(&mut self.sort)
                    .with_label("Sort by")
                    .with_options(Sort::as_vec().into_iter())
                    .show(ui, Sort::to_string);

                if ui
                    .add_enabled(self.dataset.is_loaded(), egui::Button::new("Apply sorting"))
                    .clicked()
                {
                    self.dataset
                        .load(self.filters.as_ref().unwrap(), self.sort)
                        .unwrap();
                }
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
