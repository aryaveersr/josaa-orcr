pub trait Choice: ToString + PartialEq + Clone {}

impl<T> Choice for T where T: ToString + PartialEq + Clone {}

pub struct Dropdown<'a, T, I>
where
    T: Choice,
    I: Iterator<Item = T>,
{
    label: String,
    enabled: bool,
    state: &'a mut Option<T>,
    options: Option<I>,
}

impl<'a, T, I> Dropdown<'a, T, I>
where
    T: Choice,
    I: Iterator<Item = T>,
{
    pub fn with_state(state: &'a mut Option<T>) -> Self {
        Self {
            label: "".into(),
            enabled: true,
            state,
            options: None,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }

    pub fn with_options(mut self, options: I) -> Self {
        self.options = Some(options);
        self
    }

    pub fn show(self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(&self.label);

            ui.add_enabled_ui(self.enabled, |ui| {
                egui::ComboBox::from_id_salt(&self.label)
                    // Convert selected value into string or display fallback.
                    .selected_text(match self.state {
                        Some(selected) => selected.to_string(),
                        None => "Select".into(),
                    })
                    // Show options in dropdown
                    .show_ui(ui, |ui| {
                        match self.options {
                            Some(options) => {
                                for option in options {
                                    ui.selectable_value(
                                        self.state,
                                        Some(option.clone()),
                                        option.to_string(),
                                    );
                                }
                            }

                            None => (),
                        };
                    });
            })
        });
    }
}
