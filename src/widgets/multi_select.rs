pub struct SelectOption {
    value: String,
}

impl From<String> for SelectOption {
    fn from(value: String) -> Self {
        SelectOption { value }
    }
}

pub struct Multiselect<'a, I>
where
    I: Iterator<Item = SelectOption>,
{
    label: String,
    options: Option<I>,
    state: &'a mut Vec<String>,
}

impl<'a, I> Multiselect<'a, I>
where
    I: Iterator<Item = SelectOption>,
{
    pub fn with_state(state: &'a mut Vec<String>) -> Self {
        Self {
            label: "".into(),
            options: None,
            state,
        }
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
        let options = match self.options {
            Some(opts) => opts,
            None => return,
        };

        ui.collapsing(self.label, |ui| {
            ui.horizontal_wrapped(|ui| {
                for option in options {
                    let mut checked = self.state.contains(&option.value);

                    if ui.checkbox(&mut checked, &option.value).clicked() {
                        if checked {
                            self.state.push(option.value);
                        } else {
                            self.state.retain(|i| *i != option.value);
                        }
                    }
                }
            });
        });
    }
}
