use egui::Response;
use std::collections::HashMap;

/// A widget that displays a check-box for every key
/// in a [`HashMap<String, bool>`].
///
/// # Example
/// ```
/// # use josaa_orcr::widgets::Multiselect;
/// # use std::collections::HashMap;
/// #
/// # egui::__run_test_ui(|ui| {
/// let mut state: HashMap<String, bool> = HashMap::new();
///
/// state.insert("Send me newsletters".into(), true);
/// state.insert("I agree to the terms and services".into(), false);
///
/// let response = Multiselect::with_state(&mut state)
///     .with_label("Form")
///     .show(ui);
///
/// match response {
///     Some(response) if response.changed() => {
///         println!("Multiselect state was changed.");
///     }
///
///     _ => (),
/// };
/// # });
/// ```
pub struct Multiselect<'a> {
    label: String,
    state: &'a mut HashMap<String, bool>,
    scroll: bool,
}

impl<'a> Multiselect<'a> {
    /// Create a new multi-select with the given state.
    pub fn with_state(state: &'a mut HashMap<String, bool>) -> Self {
        Self {
            label: "".into(),
            scroll: false,
            state,
        }
    }

    /// Set an label to appear to the left of the dropdown.
    /// This should be unique.
    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }

    /// Set whether to enable scrolling within the multi-select.
    pub fn with_scroll(mut self) -> Self {
        self.scroll = true;
        self
    }

    /// Render the multi-select.
    ///
    /// Returns [`None`] if it is collapsed, or the response from
    /// the checkboxes values. Use [`Response::changed()`] on this to check
    /// for state changes.
    pub fn show(self, ui: &mut egui::Ui) -> Option<Response> {
        // The container that actually contains the checkboxes.
        // Depending on `self.scroll`, this may be wrapped inside
        // a [`egui::ScrollArea`].
        let mut inner = |ui: &mut egui::Ui| {
            ui.horizontal_wrapped(|ui| {
                // Start recording the response for each checkbox value.
                let mut response = ui.response();

                for (value, checked) in self.state.iter_mut() {
                    response |= ui.checkbox(checked, value);
                }

                response
            })
            .inner
        };

        // The collapsing header that optionally wraps `inner` with a scroll area.
        egui::CollapsingHeader::new(self.label)
            .show(ui, |ui| {
                if self.scroll {
                    egui::ScrollArea::vertical().show(ui, inner).inner
                } else {
                    inner(ui)
                }
            })
            .body_response
    }
}
