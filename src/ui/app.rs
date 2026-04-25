use super::state::ApplicationState;
use crate::api::connecteduser::retrieve_connected_user;
use crate::types::{SharedProductList, SharedStoreLocationList, SharedString};
use crate::ui::pages::main;
use chimitheque_types::person::Person;
use eframe::CreationContext;
use egui::{CornerRadius, Style, Theme, vec2};
use egui_select2::select2::EguiSelect2;
use rust_i18n::t;
use std::sync::Once;
use std::sync::{Arc, Mutex};

static START: Once = Once::new();

#[derive(Default)]
pub struct App {
    // Application state.
    pub state: ApplicationState,

    // Does not work in wasm.
    // Channels for communication beetween
    // application (GUI) and worker.
    // pub sender: Option<Sender<ToWorker>>,
    // receiver: Option<Receiver<ToApp>>,

    // Selects for search form.
    pub search_store_location_widget: EguiSelect2,
    pub search_name_widget: EguiSelect2,

    // Error messages.
    pub current_error: SharedString,
    pub current_info: SharedString,

    // User information.
    pub connected_user: Arc<Mutex<Option<Person>>>,
    // Store locations.
    pub storelocations: SharedStoreLocationList,
    // Products.
    pub products: SharedProductList,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        // Does not work in wasm.
        // // Create channels.
        // let (app_tx, app_rx) = mpsc::channel();
        // let (worker_tx, worker_rx) = mpsc::channel();

        // dbg!("Spawning new worker.");

        // // Spawn a thread with a new worker.
        // let context = cc.egui_ctx.clone();
        // thread::spawn(move || {
        //     crate::worker::builder::Worker::new(worker_tx, app_rx, context).init();
        // });

        // dbg!("New worker spawned.");

        log::set_max_level(log::LevelFilter::Debug);

        // Create application state.
        let state = ApplicationState::new(&rust_i18n::locale());

        // Initialize the custom theme/styles for egui.
        setup_custom_fonts(&cc.egui_ctx);
        setup_custom_style(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Initialize select2 widgets.
        let egui_select2_translations = egui_select2::select2::Translations {
            loading: t!("select2_loading").to_string(),
            no_results: t!("select2_no_results").to_string(),
            add: t!("select2_add").to_string(),
            clear_all: t!("select2_clear_all").to_string(),
            hint: t!("select2_hint").to_string(),
        };

        let mut search_store_location = EguiSelect2::default();
        search_store_location.load_suggestions =
            Arc::new(crate::api::storelocation::load_suggestions);
        search_store_location.translations = egui_select2_translations.clone();
        search_store_location.translations.hint = t!("select2_hint_store_location").to_string();

        let mut search_name = EguiSelect2::default();
        search_name.load_suggestions = Arc::new(crate::api::name::load_suggestions);
        search_name.translations = egui_select2_translations;
        search_name.translations.hint = t!("select2_hint_name").to_string();

        // Create application.
        Self {
            state,
            // sender: Some(app_tx),
            // receiver: Some(worker_rx),
            search_store_location_widget: search_store_location,
            search_name_widget: search_name,
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Check for user informations promise.
        // if let Some(p) = &self.promise_user_info {
        //     if let Some(try_user_info) = p.ready() {
        //         match try_user_info {
        //             Ok(user_info) => {
        //                 self.user_info = Some(user_info.clone());
        //             }
        //             Err(e) => {
        //                 debug!("promise_user_info error: {e}");
        //             }
        //         }
        //         self.promise_user_info = None;
        //     }
        // }

        // Do one time startup job.
        START.call_once(|| {
            // Get connected user.
            if let Err(e) = retrieve_connected_user(Arc::clone(&self.connected_user)) {
                log::error!("retrieve_connected_user error: {e}");
            }
        });

        // Check loading state of select2 widgets.
        self.search_store_location_widget.check_loading();
        self.search_name_widget.check_loading();

        // Check channels for messages.
        // if let Some(receiver) = &self.receiver {
        //     receiver.try_recv(){
        //     }
        // }

        if self.connected_user.lock().unwrap().is_some() {
            main::ui::update(self, ui, frame);
        } else {
            egui::Panel::top("wait_user_info").show_inside(ui, |ui| ui.label(t!("wait_user_info")));
        }
    }
}

fn use_custom_accent(style: &mut Style) {
    style.visuals.widgets.active.corner_radius = CornerRadius::same(30);
    style.visuals.widgets.hovered.corner_radius = CornerRadius::same(30);
    style.visuals.widgets.inactive.corner_radius = CornerRadius::same(30);
    style.visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
    style.visuals.widgets.open.corner_radius = CornerRadius::same(30);

    style.spacing.button_padding = vec2(10., 5.);
}

fn setup_custom_style(ctx: &egui::Context) {
    // Ensure the theme is initialized
    ctx.set_theme(Theme::Light);
    ctx.style_mut_of(Theme::Light, use_custom_accent);
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Add Phosphor icons font.
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    // Install custom fonts.
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "B612-Regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "fonts/B612-Regular.ttf"
        ))),
    );

    // Start at 1 not 0 to keep the default font.
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "B612-Regular".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
