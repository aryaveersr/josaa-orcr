pub trait Choice: ToString + PartialEq + Clone {}

impl<T> Choice for T where T: ToString + PartialEq + Clone {}

pub struct Dropdown<'a, T: Choice> {
    label: String,
    enabled: bool,
    state: &'a mut Option<T>,
}

impl<'a, T: Choice> Dropdown<'a, T> {
    pub fn with_state(state: &'a mut Option<T>) -> Self {
        Self {
            label: "".into(),
            enabled: true,
            state,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }

    pub fn show<I: Iterator<Item = T>>(&mut self, ui: &mut egui::Ui, options: impl FnOnce() -> I) {
        ui.horizontal(|ui| {
            ui.label(&self.label);

            ui.add_enabled_ui(self.enabled, |ui| {
                egui::ComboBox::from_id_salt(&self.label)
                    .selected_text(match &self.state {
                        Some(selected) => selected.to_string(),
                        None => "Select".into(),
                    })
                    .show_ui(ui, |ui| {
                        for option in options() {
                            ui.selectable_value(
                                self.state,
                                Some(option.clone()),
                                option.to_string(),
                            );
                        }
                    });
            })
        });
    }
}
