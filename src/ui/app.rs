use super::state::ApplicationState;
use crate::api::connecteduser::retrieve_connected_user;
use crate::api::product::retrieve_products;
use crate::ui::pages::main;
use crate::ui::widgets::mybutton::{self, ButtonSize, ButtonVariant};
use chimitheque_types::permission::{self, Permission};
use chimitheque_types::person::Person;
use chimitheque_types::product::Product;
use chimitheque_types::requestfilter::RequestFilter;
use chimitheque_types::storelocation::StoreLocation;

use eframe::CreationContext;
use egui::{CornerRadius, Style, Theme, vec2};
use rust_i18n::t;
use std::sync::Once;
use std::sync::{Arc, Mutex};

static START: Once = Once::new();

#[derive(Default)]
pub struct App {
    // Application state.
    pub state: ApplicationState,

    // Channels for communication beetween
    // application (GUI) and worker.
    // pub sender: Option<Sender<ToWorker>>,
    // receiver: Option<Receiver<ToApp>>,

    // Error messages.
    pub current_error: Option<String>,
    pub current_info: Option<String>,

    // User information.
    pub connected_user: Arc<Mutex<Option<Person>>>,
    // Store locations.
    pub storelocations: Arc<Mutex<Option<(Vec<StoreLocation>, u64)>>>,
    // Products.
    pub products: Arc<Mutex<Option<(Vec<Product>, u64)>>>,
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
        egui_material_icons::initialize(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Create application.
        Self {
            state,
            // sender: Some(app_tx),
            // receiver: Some(worker_rx),
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

        // Check channels for messages.
        // if let Some(receiver) = &self.receiver {
        //     receiver.try_recv(){

        //     }
        // }

        // Render UI when user informations are retrieved.
        // if mybutton::mybutton(
        //     ui,
        //     format!("test {}", ICON_AUDIO_VIDEO_RECEIVER.codepoint).as_str(),
        //     ButtonSize::Md,
        //     ButtonVariant::Secondary,
        // )
        // .clicked()
        // {
        //     // let mayerr_send = self.sender.as_ref().unwrap().send(ToWorker {
        //     //     message: ToWorkerMessage::GetProducts(
        //     //         RequestFilter {
        //     //             limit: Some(10),
        //     //             ..Default::default()
        //     //         },
        //     //         Arc::clone(&self.products),
        //     //     ),
        //     // });

        //     retrieve_products(
        //         RequestFilter {
        //             limit: Some(10),
        //             ..Default::default()
        //         },
        //         Arc::clone(&self.products),
        //     );
        // };

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

    // Install custom fonts.
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "B612-Regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "fonts/B612-Regular.ttf"
        ))),
    );
    // fonts.font_data.insert(
    //     "Font_Awesome_7_Brands-Regular-400".to_owned(),
    //     std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
    //         "fonts/Font_Awesome_7_Brands-Regular-400.otf"
    //     ))),
    // );
    // fonts.font_data.insert(
    //     "Font_Awesome_7_Free-Regular-400".to_owned(),
    //     std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
    //         "fonts/Font_Awesome_7_Free-Regular-400.otf"
    //     ))),
    // );
    // fonts.font_data.insert(
    //     "Font_Awesome_7_Free-Solid-900".to_owned(),
    //     std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
    //         "fonts/Font_Awesome_7_Free-Solid-900.otf"
    //     ))),
    // );

    // Start at 1 not 0 to keep the default font.
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "B612-Regular".to_owned());
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Proportional)
    //     .or_default()
    //     .insert(2, "Font_Awesome_7_Brands-Regular-400".to_owned());
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Proportional)
    //     .or_default()
    //     .insert(3, "Font_Awesome_7_Free-Regular-400".to_owned());
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Proportional)
    //     .or_default()
    //     .insert(4, "Font_Awesome_7_Free-Solid-900".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
