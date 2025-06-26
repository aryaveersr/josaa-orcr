use std::collections::HashMap;

pub struct Multiselect<'a> {
    label: String,
    state: &'a mut HashMap<String, bool>,
}

impl<'a> Multiselect<'a> {
    pub fn with_state(state: &'a mut HashMap<String, bool>) -> Self {
        Self {
            label: "".into(),
            state,
        }
    }

    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }

    pub fn show(self, ui: &mut egui::Ui) {
        ui.collapsing(self.label, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (value, checked) in self.state.iter_mut() {
                    ui.checkbox(checked, value).clicked();
                }
            });
        });
    }
}
