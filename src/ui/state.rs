use std::collections::VecDeque;

use egui::{Pos2, Rect};
use elegance::BuiltInTheme;


// Applications pages.
#[derive(Debug, Default)]
pub enum Page {
    #[default]
    ProductList,
    ProductCreate,
    StorageList,
    StorelocationList,
    EntityList,
    Pubchem,
    PeopleList,
}

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    GetProducts(bool), // append?
    GetStorages(bool), // append?
    GetStorelocations,
    GetEntities,
    GetPubchemAutocomplete,
    GetPubchemProduct,
    GetPermissions,
    GetPeople,
    ExportProducts,
    ExportStorages,
}

/// Application state.
#[derive(Debug)]
pub struct ApplicationState {
    // The currently selected page.
    pub active_page: Page,
    // The active locale.
    pub active_locale: String,
    // The active theme.
    pub active_theme: BuiltInTheme,
    // The active tab.
    pub active_tab: usize,
    // The previous tab.
    pub previous_tab: usize,
    // Full window size and position.
    pub window_rect: Rect,
    // Top panel size and position.
    pub top_panel_rect: Rect,
    // Bottom panel size and position.
    pub bottom_panel_rect: Rect,
    // Central panel (ie. window minus top and bottombar) available size and position (without margin and padding).
    pub central_panel_rect: Rect,
    // Search form size and position.
    pub search_rect: Rect,
    // Advanced search size and position.
    pub advanced_search_rect: Rect,
    // Whether the scroll area (product and storage lists) was near the bottom.
    pub scrollarea_was_near_bottom: bool,
    // Zoom factor.
    pub zoom_factor: f32,
    // The current action.
    pub action: VecDeque<Action>,
    // Whether dark mode is enabled.
    pub darkmode: bool,
}

impl Default for ApplicationState {
    fn default() -> ApplicationState {
        Self {
            active_page: Page::ProductList,
            active_locale: String::from("fr-FR"),
            active_theme: BuiltInTheme::Paper,
            active_tab: 1,
            previous_tab: 1,
            window_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            central_panel_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            advanced_search_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            bottom_panel_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            top_panel_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            search_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 0.0, y: 0.0 },
            },
            scrollarea_was_near_bottom: false,
            action: VecDeque::new(),
            darkmode: false,
            zoom_factor: 1.0,
        }
    }
}

impl ApplicationState {
    #[must_use]
    pub fn new(active_locale: &str) -> Self {
        Self {
            active_locale: active_locale.to_string(),
            ..Default::default()
        }
    }
}
