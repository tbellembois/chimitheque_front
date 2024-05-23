use egui_aesthetix::{themes::StandardLight, Aesthetix};
use std::rc::Rc;

// Applications pages.
#[derive(Debug, Default)]
pub enum Page {
    #[default]
    ProductList,
}

/// Application state.
#[derive(Debug)]
pub struct ApplicationState {
    // The currently selected page.
    pub active_page: Page,
    // The active theme.
    pub active_theme: Rc<dyn Aesthetix>,
    // The active locale.
    pub active_locale: String,
}

impl Default for ApplicationState {
    fn default() -> ApplicationState {
        Self {
            active_page: Page::ProductList,
            active_theme: Rc::new(StandardLight),
            active_locale: String::from("fr-FR"),
        }
    }
}

impl ApplicationState {
    /// Create a new state with an active theme
    #[must_use]
    pub fn new(active_theme: Rc<dyn Aesthetix>, active_locale: &str) -> Self {
        Self {
            active_page: Page::ProductList,
            active_theme,
            active_locale: active_locale.to_string(),
        }
    }
}
