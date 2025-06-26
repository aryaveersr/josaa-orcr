use std::ops::RangeInclusive;

pub struct RangeSelector<'a> {
    label: String,

    start: u32,
    end: u32,

    bounds: &'a RangeInclusive<u32>,
    state: &'a mut RangeInclusive<u32>,
}

impl<'a> RangeSelector<'a> {
    pub fn with_state(state: &'a mut RangeInclusive<u32>, bounds: &'a RangeInclusive<u32>) -> Self {
        Self {
            label: "".into(),
            start: *state.start(),
            end: *state.end(),
            bounds,
            state,
        }
    }

    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }

    pub fn show(mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("{}: between", self.label));
            ui.add(
                egui::DragValue::new(&mut self.start)
                    .speed(1)
                    .range((*self.bounds.start())..=(self.end - 1)),
            );
            ui.label("and");
            ui.add(
                egui::DragValue::new(&mut self.end)
                    .speed(1)
                    .range((self.start + 1)..=(*self.bounds.end())),
            );

            *self.state = (self.start)..=(self.end);
        });
    }
}
