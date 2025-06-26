use egui::{Response, WidgetText};

/// A possible choice for the dropdown.
///
/// It is already implemented for all types that implement [`PartialEq`]
/// and [`Clone`], so you don't need to implement it manually.
pub trait Choice: PartialEq + Clone {}

impl<T> Choice for T where T: PartialEq + Clone {}

/// A drop-down widget.
///
/// # Example
/// ```
/// # use josaa_orcr::widgets::Dropdown;
/// #
/// # egui::__run_test_ui(|ui| {
/// let mut state = 0;
/// let mut enabled = true;
///
/// let response = Dropdown::with_state(&mut state)
///     .with_label("Number?")
///     .with_options(0..=10)
///     .with_enabled(enabled)
///     .show(ui, |i| {
///         format!("00{i}")
///     });
///
/// match response {
///     Some(response) if response.changed() => {
///         println!("Dropdown state was changed.");
///         enabled = false;
///     }
///
///     _ => (),
/// };
/// # });
/// ```
pub struct Dropdown<'a, T, I>
where
    T: Choice,
    I: Iterator<Item = T>,
{
    label: String,
    enabled: bool,
    state: &'a mut T,
    options: Option<I>,
}

impl<'a, T, I> Dropdown<'a, T, I>
where
    T: Choice,
    I: Iterator<Item = T>,
{
    /// Create a new dropdown with the given state.
    ///
    /// `T` should implement [`PartialEq`] and [`Clone`].
    pub fn with_state(state: &'a mut T) -> Self {
        Self {
            label: "".into(),
            enabled: true,
            state,
            options: None,
        }
    }

    /// Set whether the dropdown should be enabled.
    /// Default is always `true`.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set an optional label to appear to the left of the dropdown.
    pub fn with_label(mut self, label: impl AsRef<str>) -> Self {
        self.label = label.as_ref().into();
        self
    }

    /// Provide an iterator of items `T`, which are all valid values
    /// the state can take.
    pub fn with_options(mut self, options: I) -> Self {
        self.options = Some(options);
        self
    }

    /// Render the widget.
    ///
    /// `display` is a function mapping `T` into text to be displayed.
    ///
    /// Returns [`None`] if the drop-down is closed, or the response from
    /// the selectable values. Use [`Response::changed()`] on this to check
    /// for state changes.
    pub fn show<K: Into<WidgetText>>(
        self,
        ui: &mut egui::Ui,
        display: impl Fn(&T) -> K,
    ) -> Option<Response> {
        ui.horizontal(|ui| {
            // Only show the label when it isn't empty.
            if !self.label.is_empty() {
                ui.label(&self.label);
            }

            ui.add_enabled_ui(self.enabled, |ui| {
                egui::ComboBox::from_id_salt(&self.label)
                    // Convert selected value into string or display fallback.
                    .selected_text(display(&self.state))
                    // Show options in dropdown
                    .show_ui(ui, |ui| {
                        // Start recording the response for each selectable value.
                        let mut response = ui.response();
                        let options = self.options.expect("No options provided for dropdown.");

                        for option in options {
                            response |=
                                ui.selectable_value(self.state, option.clone(), display(&option));
                        }

                        response
                    })
                    .inner
            })
            .inner
        })
        .inner
    }
}
