use super::state::ApplicationState;
use crate::api::connecteduser::retrieve_connected_user;
use crate::api::product::retrieve_products;
use crate::api::storelocation::retrieve_store_locations;
use crate::defines::SEARCH_LIMIT;
use crate::types::{SharedProductAndCountList, SharedStoreLocationAndCountList, SharedString};
use crate::ui::pages::main;
use crate::ui::state::Action;
use chimitheque_types::person::Person;
use chimitheque_types::requestfilter::RequestFilter;
use eframe::CreationContext;
use egui::{Style, Theme};
use egui_select2::select2::EguiSelect2;
use rust_i18n::t;
use std::sync::Once;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_rs_shared_channel::spsc::{Receiver, Sender, channel};

static START: Once = Once::new();

#[derive(Default, PartialEq)]
pub enum ProductType {
    Chemical,
    Biological,
    Consumable,
    #[default]
    All,
}

#[derive(Default)]
pub struct App {
    // Application state.
    pub state: ApplicationState,

    // Channels for info and error messages.
    pub info_sender: Option<Sender<String>>,
    pub error_sender: Option<Sender<String>>,
    pub info_receiver: Option<Receiver<String>>,
    pub error_receiver: Option<Receiver<String>>,

    // Channels for loading state.
    pub loading_sender: Option<Sender<bool>>,
    pub loading_receiver: Option<Receiver<bool>>,

    // Does not work in wasm.
    // Channels for communication beetween
    // application (GUI) and worker.
    // pub sender: Option<Sender<ToWorker>>,
    // receiver: Option<Receiver<ToApp>>,
    pub search_form_expanded: bool,

    // Widgets/variables for search form.
    pub search_part_of_name: String,
    pub search_barecode: String,

    pub search_store_location_widget: EguiSelect2, // EguiSelect2 contains its own variable.
    pub search_name_widget: EguiSelect2,
    pub search_entity_widget: EguiSelect2,
    pub search_producer_ref_widget: EguiSelect2,
    pub search_signal_word_widget: EguiSelect2,
    pub search_category_widget: EguiSelect2,
    pub search_cas_number_widget: EguiSelect2,
    pub search_empirical_formula_widget: EguiSelect2,
    pub search_hazard_statement_widget: EguiSelect2,
    pub search_precautionary_statement_widget: EguiSelect2,
    pub search_symbol_widget: EguiSelect2,
    pub search_tag_widget: EguiSelect2,

    pub search_product_cmr: bool,
    pub search_product_to_destroy: bool,
    pub search_product_borrowed: bool,
    pub search_product_type: ProductType,

    // Error messages.
    pub current_error: Option<String>,
    pub current_info: Option<String>,

    // User information.
    pub connected_user: Arc<Mutex<Option<Person>>>,
    // Store locations.
    pub storelocations: SharedStoreLocationAndCountList,
    // Products.
    pub products: SharedProductAndCountList,

    // Current search offset.
    pub current_search_offset: usize,

    // Loading state.
    pub is_loading: bool,
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

        // Create info, error and loading state channels.
        let info_channel = channel::<String>(2048);
        let error_channel = channel::<String>(2048);

        let (info_sender, info_receiver) = info_channel.split();
        let (error_sender, error_receiver) = error_channel.split();

        let loading_channel = channel::<bool>(1024);
        let (loading_sender, loading_receiver) = loading_channel.split();

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
            load_more: t!("select2_load_more").to_string(),
            hint: t!("select2_hint").to_string(),
        };

        let mut search_store_location = EguiSelect2::default();
        search_store_location.load_suggestions =
            Arc::new(crate::api::storelocation::load_suggestions);
        search_store_location.translations = egui_select2_translations.clone();
        search_store_location.translations.hint = t!("select2_hint_store_location").to_string();

        let mut search_name = EguiSelect2::default();
        search_name.load_suggestions = Arc::new(crate::api::name::load_suggestions);
        search_name.translations = egui_select2_translations.clone();
        search_name.translations.hint = t!("select2_hint_name").to_string();

        let mut search_entity = EguiSelect2::default();
        search_entity.load_suggestions = Arc::new(crate::api::entity::load_suggestions);
        search_entity.translations = egui_select2_translations.clone();
        search_entity.translations.hint = t!("select2_hint_entity").to_string();

        let mut search_cas_number = EguiSelect2::default();
        search_cas_number.load_suggestions = Arc::new(crate::api::casnumber::load_suggestions);
        search_cas_number.translations = egui_select2_translations.clone();
        search_cas_number.translations.hint = t!("select2_hint_cas_number").to_string();

        let mut search_ce_number = EguiSelect2::default();
        search_ce_number.load_suggestions = Arc::new(crate::api::cenumber::load_suggestions);
        search_ce_number.translations = egui_select2_translations.clone();
        search_ce_number.translations.hint = t!("select2_hint_ce_number").to_string();

        let mut search_empirical_formula = EguiSelect2::default();
        search_empirical_formula.load_suggestions =
            Arc::new(crate::api::empiricalformula::load_suggestions);
        search_empirical_formula.translations = egui_select2_translations.clone();
        search_empirical_formula.translations.hint =
            t!("select2_hint_empirical_formula").to_string();

        let mut search_hazard_statement = EguiSelect2::default();
        search_hazard_statement.load_suggestions =
            Arc::new(crate::api::hazardstatement::load_suggestions);
        search_hazard_statement.translations = egui_select2_translations.clone();
        search_hazard_statement.translations.hint = t!("select2_hint_hazard_statement").to_string();
        search_hazard_statement.multiple = true;

        let mut search_precautionary_statement = EguiSelect2::default();
        search_precautionary_statement.load_suggestions =
            Arc::new(crate::api::precautionarystatement::load_suggestions);
        search_precautionary_statement.translations = egui_select2_translations.clone();
        search_precautionary_statement.translations.hint =
            t!("select2_hint_precautionary_statement").to_string();
        search_precautionary_statement.multiple = true;

        let mut search_symbol = EguiSelect2::default();
        search_symbol.load_suggestions = Arc::new(crate::api::symbol::load_suggestions);
        search_symbol.format_suggestion = Box::new(crate::api::symbol::format_suggestion);
        search_symbol.translations = egui_select2_translations.clone();
        search_symbol.translations.hint = t!("select2_hint_symbol").to_string();
        search_symbol.multiple = true;

        let mut search_tag = EguiSelect2::default();
        search_tag.load_suggestions = Arc::new(crate::api::tag::load_suggestions);
        search_tag.translations = egui_select2_translations.clone();
        search_tag.translations.hint = t!("select2_hint_tag").to_string();

        let mut search_signal_word = EguiSelect2::default();
        search_signal_word.load_suggestions = Arc::new(crate::api::signalword::load_suggestions);
        search_signal_word.translations = egui_select2_translations.clone();
        search_signal_word.translations.hint = t!("select2_hint_signal_word").to_string();

        let mut search_producer_ref = EguiSelect2::default();
        search_producer_ref.load_suggestions = Arc::new(crate::api::producerref::load_suggestions);
        search_producer_ref.translations = egui_select2_translations.clone();
        search_producer_ref.translations.hint = t!("select2_hint_producer_ref").to_string();

        let mut search_category = EguiSelect2::default();
        search_category.load_suggestions = Arc::new(crate::api::category::load_suggestions);
        search_category.translations = egui_select2_translations.clone();
        search_category.translations.hint = t!("select2_hint_category").to_string();

        // Create application.
        Self {
            state,
            // sender: Some(app_tx),
            // receiver: Some(worker_rx),
            search_store_location_widget: search_store_location,
            search_name_widget: search_name,
            search_entity_widget: search_entity,
            search_producer_ref_widget: search_producer_ref,
            search_signal_word_widget: search_signal_word,
            search_tag_widget: search_tag,
            search_cas_number_widget: search_cas_number,
            search_category_widget: search_category,
            search_empirical_formula_widget: search_empirical_formula,
            search_hazard_statement_widget: search_hazard_statement,
            search_precautionary_statement_widget: search_precautionary_statement,
            search_symbol_widget: search_symbol,
            search_form_expanded: true,
            info_sender: Some(info_sender),
            error_sender: Some(error_sender),
            info_receiver: Some(info_receiver),
            error_receiver: Some(error_receiver),
            loading_sender: Some(loading_sender),
            loading_receiver: Some(loading_receiver),
            ..Default::default()
        }
    }

    pub fn GetRequestFilter(&self) -> RequestFilter {
        let mut filter = RequestFilter::default();

        filter.custom_name_part_of =
            (!self.search_part_of_name.is_empty()).then(|| self.search_part_of_name.clone());
        filter.storage_barecode =
            (!self.search_barecode.is_empty()).then(|| self.search_barecode.clone());
        filter.store_location = self
            .search_store_location_widget
            .selected
            .first()
            .and_then(|s| s.id);
        filter.name = self.search_name_widget.selected.first().and_then(|s| s.id);
        filter.entity = self
            .search_entity_widget
            .selected
            .first()
            .and_then(|s| s.id);
        filter.producer_ref = self
            .search_producer_ref_widget
            .selected
            .first()
            .and_then(|s| s.id);
        filter.signal_word = self
            .search_signal_word_widget
            .selected
            .first()
            .and_then(|s| s.id);
        filter.category = self
            .search_category_widget
            .selected
            .first()
            .and_then(|s| s.id);
        filter.cas_number = self
            .search_cas_number_widget
            .selected
            .first()
            .and_then(|s| s.id);
        filter.empirical_formula = self
            .search_empirical_formula_widget
            .selected
            .first()
            .and_then(|s| s.id);

        let hazard_statements: Vec<_> = self
            .search_hazard_statement_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        filter.hazard_statements = (!hazard_statements.is_empty()).then_some(hazard_statements);

        let precautionary_statements: Vec<_> = self
            .search_precautionary_statement_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        filter.precautionary_statements =
            (!precautionary_statements.is_empty()).then_some(precautionary_statements);

        let symbols: Vec<_> = self
            .search_symbol_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        filter.symbols = (!symbols.is_empty()).then_some(symbols);

        let tags: Vec<_> = self
            .search_tag_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        filter.tags = (!tags.is_empty()).then_some(tags);

        filter.is_cmr = self.search_product_cmr;
        filter.storage_to_destroy = self.search_product_to_destroy;
        filter.borrowing = self.search_product_borrowed;
        match self.search_product_type {
            ProductType::Chemical => {
                filter.show_chem = true;
            }
            ProductType::Biological => {
                filter.show_bio = true;
            }
            ProductType::Consumable => {
                filter.show_consu = true;
            }
            ProductType::All => (),
        }

        filter.limit = Some(SEARCH_LIMIT);
        filter.offset = Some(self.current_search_offset as u64);

        filter
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

        // Check channels messages.
        if let Some(info_receiver) = self.info_receiver.as_ref() {
            if let Ok(info) = info_receiver.recv(Some(Duration::ZERO)) {
                self.current_info = info.into();
            }
        }
        if let Some(error_receiver) = self.error_receiver.as_ref() {
            if let Ok(error) = error_receiver.recv(Some(Duration::ZERO)) {
                self.current_error = error.into();
            }
        }
        if let Some(loading_receiver) = self.loading_receiver.as_ref() {
            if let Ok(loading) = loading_receiver.recv(Some(Duration::ZERO)) {
                self.is_loading = loading.unwrap_or_default();
            }
        }

        // Handle action.
        match self.state.action {
            Action::GetProducts => {
                retrieve_products(
                    &self.GetRequestFilter(),
                    Arc::clone(&self.products),
                    false,
                    self.info_sender.clone(),
                    self.error_sender.clone(),
                    self.loading_sender.clone(),
                );

                self.state.action = Action::None;
            }
            Action::GetStorelocations => {
                retrieve_store_locations(
                    &RequestFilter {
                        limit: Some(SEARCH_LIMIT),
                        ..Default::default()
                    },
                    Arc::clone(&self.storelocations),
                    false,
                    self.info_sender.clone(),
                    self.error_sender.clone(),
                    self.loading_sender.clone(),
                );

                self.state.action = Action::None;
            }
            Action::None => {}
        }

        // Update window size.
        self.state.window_rect = ui.max_rect();

        // Check loading state of select2 widgets.
        self.search_store_location_widget.check_loading();
        self.search_name_widget.check_loading();
        self.search_entity_widget.check_loading();
        self.search_category_widget.check_loading();
        self.search_empirical_formula_widget.check_loading();
        self.search_hazard_statement_widget.check_loading();
        self.search_precautionary_statement_widget.check_loading();
        self.search_symbol_widget.check_loading();
        self.search_tag_widget.check_loading();
        self.search_cas_number_widget.check_loading();
        self.search_producer_ref_widget.check_loading();
        self.search_signal_word_widget.check_loading();

        // Check channels for messages.
        // if let Some(receiver) = &self.receiver {
        //     receiver.try_recv(){
        //     }
        // }

        let ctx = ui.ctx();

        // Foreground loading overlay.
        if self.is_loading {
            let screen_rect = ctx.screen_rect();

            // keep repainting so spinner animates
            ctx.request_repaint_after(std::time::Duration::from_millis(16));

            egui::Area::new("spinner_overlay".into())
                .order(egui::Order::Foreground)
                .fixed_pos(screen_rect.center() - egui::vec2(20.0, 20.0))
                .interactable(false)
                .show(ctx, |ui| {
                    ui.add(egui::Spinner::new().size(40.0));
                });
        }

        if self.connected_user.lock().unwrap().is_some() {
            main::ui::update(self, ui, frame);
        } else {
            egui::Panel::top("wait_user_info").show_inside(ui, |ui| ui.label(t!("wait_user_info")));
        }
    }
}

fn use_custom_accent(style: &mut Style) {
    style.visuals.widgets.active.corner_radius =
        crate::defines::VISUALS_WIDGETS_ACTIVE_CORNER_RADIUS;
    style.visuals.widgets.hovered.corner_radius =
        crate::defines::VISUALS_WIDGETS_HOVERED_CORNER_RADIUS;
    style.visuals.widgets.inactive.corner_radius =
        crate::defines::VISUALS_WIDGETS_INACTIVE_CORNER_RADIUS;
    style.visuals.widgets.noninteractive.corner_radius =
        crate::defines::VISUALS_WIDGETS_NONINTERACTIVE_CORNER_RADIUS;
    style.visuals.widgets.open.corner_radius = crate::defines::VISUALS_WIDGETS_OPEN_CORNER_RADIUS;

    style.override_font_id = Some(egui::FontId::proportional(crate::defines::GLOBAL_FONT_SIZE));
}

fn setup_custom_style(ctx: &egui::Context) {
    // Ensure the theme is initialized.
    ctx.set_theme(Theme::Light);
    ctx.style_mut_of(Theme::Light, use_custom_accent);
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Font data.
    fonts.font_data.insert(
        "B612".into(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "fonts/B612-Regular.ttf"
        ))),
    );

    fonts.font_data.insert(
        "phosphor-fill".into(),
        Arc::new(egui_phosphor::Variant::Fill.font_data()),
    );

    fonts.font_data.insert(
        "phosphor".into(),
        Arc::new(egui_phosphor::Variant::Regular.font_data()),
    );

    // Proportional family
    let proportional = fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap();

    // proportional.insert(0, "B612".into());
    proportional.push("phosphor-fill".into());
    proportional.push("phosphor".into());

    fonts.families.insert(
        egui::FontFamily::Name("phosphor-fill".into()),
        vec!["phosphor-fill".into()],
    );
    fonts.families.insert(
        egui::FontFamily::Name("phosphor".into()),
        vec!["phosphor".into()],
    );

    ctx.set_fonts(fonts);
}
