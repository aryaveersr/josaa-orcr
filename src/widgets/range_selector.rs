use crate::types::RankRange;
use egui::Widget;

/// A widget that allows inputting a [`RankRange`]
/// using two [`egui::DragValue`]s.
///
/// # Example
/// ```
/// # use josaa_orcr::{widgets::RangeSelector, types::RankRange};
/// # use egui::Widget;
/// #
/// # egui::__run_test_ui(|ui| {
/// let mut state = RankRange::new(2, 4);
/// let bounds = RankRange::new(1, 12);
///
/// let response = RangeSelector::with_state(&mut state, &bounds)
///     .with_label("Enter a range")
///     .ui(ui);
///
/// if response.changed() {
///     println!("Range selector state was changed.");
/// }
/// # });
/// ```
pub struct RangeSelector<'a> {
    label: String,
    bounds: &'a RankRange,
    state: &'a mut RankRange,
}

impl<'a> RangeSelector<'a> {
    /// Create a new range selector with the given state and bounds.
    pub fn with_state(state: &'a mut RankRange, bounds: &'a RankRange) -> Self {
        Self {
            label: "".into(),
            bounds,
            state,
        }
    }

    /// Set an optional label to appear to the left of the range selector.
    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }
}

impl Widget for RangeSelector<'_> {
    /// Render the range selector.
    ///
    /// Use [`Response::changed`] on the returned response to
    /// check for state changes.
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            // Only show the label when it isn't empty.
            if !self.label.is_empty() {
                ui.label(self.label);
            }

            // Record response from the first input.
            let mut response = egui::DragValue::new(&mut self.state.start)
                .speed(1)
                // Can't go below `self.bounds.start` or above `self.state.end`.
                .range((self.bounds.start)..=(self.state.end - 1))
                .ui(ui);

            // Record response from second input
            response |= egui::DragValue::new(&mut self.state.end)
                .speed(1)
                // Can't go below `self.state.start` or above `self.bounds.end`.
                .range((self.state.start + 1)..=(self.bounds.end))
                .ui(ui);

            response
        })
        .inner
    }
}
