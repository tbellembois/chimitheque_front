use super::state::ApplicationState;
use crate::{
    api::{self},
    error::apperror::AppError,
    ui::pages::main,
};
use chimitheque_types::userinfo::UserInfo;
use eframe::CreationContext;
use egui_aesthetix::{
    themes::{CarlDark, StandardDark, StandardLight},
    Aesthetix,
};
use log::{debug, error, info};
use poll_promise::Promise;
use rust_i18n::t;
use std::{rc::Rc, sync::Once};

static START: Once = Once::new();

#[derive(Default)]
pub struct App {
    // Application state.
    pub state: ApplicationState,

    // Holds the supported themes that the user can switch between.
    pub themes: Vec<Rc<dyn Aesthetix>>,

    // Current error if one.
    pub current_error: Option<AppError>,
    // Current info if one.
    pub current_info: Option<String>,

    // User information.
    pub user_info: Option<UserInfo>,

    // Request user info promise.
    pub promise_user_info: Option<Promise<Result<UserInfo, AppError>>>,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        // Load custom fonts and styles.
        setup_custom_fonts(&cc.egui_ctx);

        // Load themes. Default is the first one.
        let themes: Vec<Rc<dyn Aesthetix>> = vec![
            Rc::new(StandardLight),
            Rc::new(StandardDark),
            Rc::new(CarlDark),
        ];
        let active_theme: Rc<dyn Aesthetix> = match themes.first() {
            Some(theme) => theme.clone(),
            None => panic!("The first theme in the list of available themes could not be loaded."),
        };

        // Create application state.
        let state = ApplicationState::new(active_theme, &rust_i18n::locale());

        // Initialize the custom theme/styles for egui.
        cc.egui_ctx.set_style(state.active_theme.custom_style());

        // Create application.
        App {
            state,
            themes,
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Check for user informations promise.
        if let Some(p) = &self.promise_user_info {
            if let Some(try_user_info) = p.ready() {
                match try_user_info {
                    Ok(user_info) => {
                        self.user_info = Some(user_info.clone());
                    }
                    Err(e) => {
                        debug!("promise_user_info error: {e}");
                    }
                }
                self.promise_user_info = None;
            }
        }

        // Do one time startup job.
        START.call_once(|| {
            // Get user informations.
            self.promise_user_info = Some(api::userinfo::retrieve_userinfo(ctx));
        });

        // Render UI when user informations are retrieved.
        if self.user_info.is_some() {
            main::ui::update(self, ctx, frame);
        } else {
            egui::TopBottomPanel::top("wait_user_info")
                .show(ctx, |ui| ui.label(t!("wait_user_info")));
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install custom fonts.
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Font-Awesome-6-Brands-Regular-400".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "fonts/Font-Awesome-6-Brands-Regular-400.otf"
        )),
    );
    fonts.font_data.insert(
        "Font-Awesome-6-Free-Regular-400".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/Font-Awesome-6-Free-Regular-400.otf")),
    );
    fonts.font_data.insert(
        "Font-Awesome-6-Free-Solid-900".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/Font-Awesome-6-Free-Solid-900.otf")),
    );

    // Start at 1 not 0 to keep the default font.
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "Font-Awesome-6-Brands-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(2, "Font-Awesome-6-Free-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(3, "Font-Awesome-6-Free-Solid-900".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
