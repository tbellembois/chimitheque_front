use egui::{Pos2, Rect};

// Applications pages.
#[derive(Debug, Default)]
pub enum Page {
    #[default]
    ProductList,
    StorelocationList,
    EntityList,
    Pubchem,
}

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    GetProducts,
    GetStorelocations,
    GetEntities,
    GetPubchemAutocomplete,
    GetPubchemProduct,
    GetPermissions,
}

/// Application state.
#[derive(Debug)]
pub struct ApplicationState {
    // The currently selected page.
    pub active_page: Page,
    // The active locale.
    pub active_locale: String,
    // Window size and position.
    pub window_rect: Rect,
    // Advanced search size and position.
    pub advanced_search_rect: Rect,
    // Top panel size and position.
    pub top_panel_rect: Rect,
    // Whether the scroll area was near the bottom.
    pub scrollarea_was_near_bottom: bool,
    // The current action.
    pub action: Action,
    // Whether dark mode is enabled.
    pub darkmode: bool,
}

impl Default for ApplicationState {
    fn default() -> ApplicationState {
        Self {
            active_page: Page::ProductList,
            active_locale: String::from("fr-FR"),
            window_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            advanced_search_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            top_panel_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            scrollarea_was_near_bottom: false,
            action: Action::None,
            darkmode: false,
        }
    }
}

impl ApplicationState {
    #[must_use]
    pub fn new(active_locale: &str) -> Self {
        Self {
            active_page: Page::ProductList,
            active_locale: active_locale.to_string(),
            window_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            advanced_search_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            top_panel_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            scrollarea_was_near_bottom: false,
            action: Action::None,
            darkmode: false,
        }
    }
}
