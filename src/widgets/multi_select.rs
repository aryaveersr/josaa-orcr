use std::collections::HashMap;

pub struct Multiselect<'a> {
    label: String,
    state: &'a mut HashMap<String, bool>,
    scroll: bool,
}

impl<'a> Multiselect<'a> {
    pub fn with_state(state: &'a mut HashMap<String, bool>) -> Self {
        Self {
            label: "".into(),
            scroll: false,
            state,
        }
    }

    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }

    pub fn with_scroll(mut self) -> Self {
        self.scroll = true;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) {
        let mut inner = |ui: &mut egui::Ui| {
            ui.horizontal_wrapped(|ui| {
                for (value, checked) in self.state.iter_mut() {
                    ui.checkbox(checked, value).clicked();
                }
            });
        };

        egui::CollapsingHeader::new(self.label).show(ui, |ui| {
            if self.scroll {
                egui::ScrollArea::vertical().show(ui, inner);
            } else {
                inner(ui);
            }
        });
    }
}
