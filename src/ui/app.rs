use super::state::ApplicationState;
use crate::api::connecteduser::get_connected_user;
use crate::api::entity::get_entities;
use crate::api::people::get_people;
use crate::api::permission::get_permissions;
use crate::api::product::get_products;
use crate::api::pubchemproduct::get_pubchem_product;
use crate::api::pubchemsearch::get_pubchem_autocomplete;
use crate::api::storage::{export_storages, get_storages};
use crate::api::storelocation::get_store_locations;
use crate::defines::SEARCH_LIMIT;
use crate::download::download_csv;
use crate::types::{
    GenericOrder, Permission, PermissionStatus, ProductType, ProductsOrderBy,
    SharedEntityAndCountList, SharedPermissionList, SharedPersonAndCountList,
    SharedProductAndCountList, SharedPubchemAutocomplete, SharedPubchemProduct,
    SharedStorageAndCountList, SharedStoreLocationAndCountList, SharedString, StoragesOrderBy,
    StoreLocationsOrderBy,
};
use crate::ui::pages::main;
use crate::ui::state::Action;
use crate::ui::validators;
use crate::{atomic, elog};
use chimitheque_types::entity::Entity;
use chimitheque_types::permission::PermissionItem;
use chimitheque_types::person::Person;
use chimitheque_types::product::Product;
use chimitheque_types::pubchem::Autocomplete;
use chimitheque_types::pubchemproduct::PubchemProduct;
use chimitheque_types::requestfilter::RequestFilter;
use chimitheque_types::storage::Storage;
use chimitheque_types::storelocation::StoreLocation;
use eframe::CreationContext;
use egui::{Style, TextureHandle, Theme};
use egui_select2::select2::EguiSelect2;
use rust_i18n::t;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Once;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_rs_shared_channel::spsc::{Receiver, Sender, channel};

static START: Once = Once::new();

#[derive(Default)]
pub struct App {
    // Application state.
    pub state: ApplicationState,

    // Channel for communication between application and functions.
    pub channel_sender: Option<Sender<bool>>,
    pub channel_receiver: Option<Receiver<bool>>,

    // Image textures.
    pub textures: HashMap<String, TextureHandle>,

    // Does not work in wasm.
    // Channels for communication beetween
    // application (GUI) and worker.
    // pub sender: Option<Sender<ToWorker>>,
    // receiver: Option<Receiver<ToApp>>,

    // Product ids of cards shown (ie. expanded) in the product list.
    pub product_cards_shown: Vec<u64>,
    pub product_cards_actions_shown: Vec<u64>,
    // Storage ids of cards shown (ie. expanded) in the storage list.
    pub storage_cards_shown: Vec<u64>,
    pub storage_cards_actions_shown: Vec<u64>,

    // Is the search form expanded?
    pub search_form_expanded: bool,
    // Is the pubchem search results expanded?
    pub pubchem_results_expanded: bool,

    // Widgets/variables for principal search form.
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

    // Widgets/variables for create product form.
    pub create_product_type: ProductType,
    pub create_product_tag_widget: EguiSelect2,
    pub create_product_category_widget: EguiSelect2,
    pub create_product_name_widget: EguiSelect2,
    pub create_product_synonym_widget: EguiSelect2,
    pub create_product_empirical_formula_widget: EguiSelect2,
    pub create_product_linear_formula_widget: EguiSelect2,
    pub create_product_cas_number_widget: EguiSelect2,
    pub create_product_ce_number_widget: EguiSelect2,
    pub create_product_specificity: String,
    pub create_product_inchi: String,
    pub create_product_inchikey: String,
    pub create_product_smiles: String,
    pub create_product_molecular_weight: String,
    pub create_product_unit_widget: EguiSelect2,
    pub create_product_3d_formula: String,
    pub create_product_molecule_picture: Vec<u8>,
    // // Works on both native and WASM
    // let file = rfd::AsyncFileDialog::new()
    //     .add_filter("text", &["txt", "rs"])
    //     .pick_file()
    //     .await;

    // if let Some(file) = file {
    //     // On WASM, you must read the content into bytes
    //     // On native, you can also access file.path()
    //     let data: Vec<u8> = file.read().await;
    // }
    pub create_product_msds_link: String,
    pub create_product_producer_sheet: String,
    pub create_product_physical_state_widget: EguiSelect2,
    pub create_product_class_of_compound_widget: EguiSelect2,
    pub create_product_signal_word_widget: EguiSelect2,
    pub create_product_symbol_widget: EguiSelect2,
    pub create_product_hazard_statement_widget: EguiSelect2,
    pub create_product_precautionary_statement_widget: EguiSelect2,
    pub create_product_disposal_comment: String,
    pub create_product_remark: String,
    pub create_product_restricted: bool,
    pub create_product_radioactive: bool,

    // Widgets/variables for the store location page.
    pub search_store_location: String,
    pub search_store_location_last_edit: f64,
    pub search_store_location_action_triggered: bool,

    // Widgets/variables for the entity page.
    pub search_entity: String,
    pub search_entity_last_edit: f64,
    pub search_entity_action_triggered: bool,

    // Widgets/variables for the person page.
    pub search_person: String,
    pub search_person_last_edit: f64,
    pub search_person_action_triggered: bool,

    // Widgets/variables for pubchem.
    pub pubchem_search: String,
    pub pubchem_search_name_clicked: String,

    // User information.
    pub connected_user: Arc<Mutex<Option<Person>>>,
    // Store locations.
    pub store_locations: SharedStoreLocationAndCountList,
    // Entities.
    pub entities: SharedEntityAndCountList,
    // People.
    pub people: SharedPersonAndCountList,
    // Products.
    pub products: SharedProductAndCountList,
    // Storages.
    pub storages: SharedStorageAndCountList,
    // Pubchem autocomplete.
    pub pubchem_autocomplete: SharedPubchemAutocomplete,
    // Pubchem product selected.
    pub pubchem_product: SharedPubchemProduct,
    // Permissions.
    pub permissions: SharedPermissionList,
    // Export storages.
    pub export_storages: SharedString,

    // Sorting for store locations.
    pub store_locations_order_by: StoreLocationsOrderBy,
    pub store_locations_order: GenericOrder,

    // Sorting for products.
    pub products_order_by: ProductsOrderBy,
    pub products_order: GenericOrder,

    // Sorting for storages.
    pub storages_order_by: StoragesOrderBy,
    pub storages_order: GenericOrder,
    pub storages_show_archives: bool,

    // Sorting for entities.
    pub entities_order: GenericOrder,

    // Sorting for people.
    pub people_order: GenericOrder,

    // Current search offset.
    pub current_search_offset: usize,

    // Loading state.
    pub is_loading: bool,
}

impl App {
    /// # Panics
    ///
    /// Will panic if svg pictures cannot be loaded, or if custom fonts cannot be set.
    #[allow(clippy::unwrap_used)]
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

        // Create application channel.
        let channel_channel = channel::<bool>(1024);
        let (channel_sender, channel_receiver) = channel_channel.split();

        // Create application state.
        let state = ApplicationState::new(&rust_i18n::locale());

        // Initialize the custom theme/styles for egui.
        setup_custom_fonts(&cc.egui_ctx).unwrap();
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

        // .. for search form
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

        // .. for create product form
        let mut create_product_tag = EguiSelect2::default();
        create_product_tag.multiple = true;
        create_product_tag.read_only = false;
        create_product_tag.show_border = true;
        create_product_tag.load_suggestions = Arc::new(crate::api::tag::load_suggestions);
        create_product_tag.translations = egui_select2_translations.clone();
        create_product_tag.translations.hint = t!("select2_hint_tag").to_string();

        let mut create_product_category = EguiSelect2::default();
        create_product_category.read_only = false;
        create_product_category.show_border = true;
        create_product_category.load_suggestions = Arc::new(crate::api::category::load_suggestions);
        create_product_category.translations = egui_select2_translations.clone();
        create_product_category.translations.hint = t!("select2_hint_category").to_string();

        let mut create_product_name = EguiSelect2::default();
        create_product_name.read_only = false;
        create_product_name.show_border = true;
        create_product_name.validate_new_item = Some(Arc::new(validators::name::validate));
        create_product_name.load_suggestions = Arc::new(crate::api::name::load_suggestions);
        create_product_name.translations = egui_select2_translations.clone();
        create_product_name.translations.hint = t!("select2_hint_name").to_string();

        let mut create_product_synonym = EguiSelect2::default();
        create_product_synonym.read_only = false;
        create_product_synonym.show_border = true;
        create_product_synonym.multiple = true;
        create_product_synonym.validate_new_item = Some(Arc::new(validators::name::validate));
        create_product_synonym.load_suggestions = Arc::new(crate::api::name::load_suggestions);
        create_product_synonym.translations = egui_select2_translations.clone();
        create_product_synonym.translations.hint = t!("select2_hint_synonym").to_string();

        let mut create_product_empirical_formula = EguiSelect2::default();
        create_product_empirical_formula.read_only = false;
        create_product_empirical_formula.show_border = true;
        create_product_empirical_formula.load_suggestions =
            Arc::new(crate::api::empiricalformula::load_suggestions);
        create_product_empirical_formula.translations = egui_select2_translations.clone();
        create_product_empirical_formula.translations.hint =
            t!("select2_hint_empirical_formula").to_string();

        let mut create_product_linear_formula = EguiSelect2::default();
        create_product_linear_formula.read_only = false;
        create_product_linear_formula.show_border = true;
        create_product_linear_formula.load_suggestions =
            Arc::new(crate::api::linearformula::load_suggestions);
        create_product_linear_formula.translations = egui_select2_translations.clone();
        create_product_linear_formula.translations.hint =
            t!("select2_hint_linear_formula").to_string();

        // Initialize textures.
        let mut textures = HashMap::new();
        textures.insert(
            "chimitheque_logo_light".to_string(),
            load_svg_texture(
                &cc.egui_ctx,
                "chimitheque_logo_light",
                include_bytes!("../assets/chimitheque_logo_light.svg"),
            )
            .unwrap(),
        );
        textures.insert(
            "chimitheque_logo_dark".to_string(),
            load_svg_texture(
                &cc.egui_ctx,
                "chimitheque_logo_dark",
                include_bytes!("../assets/chimitheque_logo_dark.svg"),
            )
            .unwrap(),
        );
        textures.insert(
            "flag_fr".to_string(),
            load_svg_texture(&cc.egui_ctx, "flag_fr", include_bytes!("../assets/fr.svg")).unwrap(),
        );
        textures.insert(
            "flag_gb".to_string(),
            load_svg_texture(&cc.egui_ctx, "flag_gb", include_bytes!("../assets/gb.svg")).unwrap(),
        );
        textures.insert(
            "ghs01".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs01", include_bytes!("../assets/GHS01.svg")).unwrap(),
        );
        textures.insert(
            "ghs02".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs02", include_bytes!("../assets/GHS02.svg")).unwrap(),
        );
        textures.insert(
            "ghs03".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs03", include_bytes!("../assets/GHS03.svg")).unwrap(),
        );
        textures.insert(
            "ghs04".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs04", include_bytes!("../assets/GHS04.svg")).unwrap(),
        );
        textures.insert(
            "ghs05".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs05", include_bytes!("../assets/GHS05.svg")).unwrap(),
        );
        textures.insert(
            "ghs06".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs06", include_bytes!("../assets/GHS06.svg")).unwrap(),
        );
        textures.insert(
            "ghs07".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs07", include_bytes!("../assets/GHS07.svg")).unwrap(),
        );
        textures.insert(
            "ghs08".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs08", include_bytes!("../assets/GHS08.svg")).unwrap(),
        );
        textures.insert(
            "ghs09".to_string(),
            load_svg_texture(&cc.egui_ctx, "ghs09", include_bytes!("../assets/GHS09.svg")).unwrap(),
        );

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
            search_form_expanded: false,

            create_product_tag_widget: create_product_tag,
            create_product_category_widget: create_product_category,
            create_product_name_widget: create_product_name,
            create_product_synonym_widget: create_product_synonym,
            create_product_empirical_formula_widget: create_product_empirical_formula,
            create_product_linear_formula_widget: create_product_linear_formula,

            channel_sender: Some(channel_sender),
            channel_receiver: Some(channel_receiver),
            textures,
            ..Default::default()
        }
    }

    pub fn has_permission(
        &mut self,
        permission_item: &PermissionItem,
        permission_entity: Option<u64>,
        http_method: &ehttp::Method,
        shared_permissions: &SharedPermissionList,
    ) -> bool {
        let mut permissions_lock = match shared_permissions.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return false;
            }
        };

        // if let Some(mut permissions) = maybe_permissions {
        let maybe_permission = permissions_lock.iter().find(|p| {
            p.http_method == *http_method
                && p.item == *permission_item
                && p.entity == permission_entity
        });

        if let Some(permission) = maybe_permission {
            if permission.status == PermissionStatus::Done {
                return permission.granted;
            }

            return false;
        }
        permissions_lock.push(Permission {
            unique_id: atomic::get_next_id(),
            status: PermissionStatus::ToRetrieve,
            item: permission_item.clone(),
            entity: permission_entity,
            http_method: http_method.clone(),
            granted: false,
        });

        self.state.action.push_back(Action::GetPermissions);

        false
    }

    #[must_use]
    pub fn get_request_filter(&self) -> RequestFilter {
        let hazard_statements: Vec<_> = self
            .search_hazard_statement_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let precautionary_statements: Vec<_> = self
            .search_precautionary_statement_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let symbols: Vec<_> = self
            .search_symbol_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let tags: Vec<_> = self
            .search_tag_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let mut filter = RequestFilter {
            offset: Some(self.current_search_offset),
            limit: Some(SEARCH_LIMIT),
            borrowing: self.search_product_borrowed,
            cas_number: self
                .search_cas_number_widget
                .selected
                .first()
                .and_then(|s| s.id),
            is_cmr: self.search_product_cmr,
            category: self
                .search_category_widget
                .selected
                .first()
                .and_then(|s| s.id),
            custom_name_part_of: (!self.search_part_of_name.is_empty())
                .then(|| self.search_part_of_name.clone()),
            empirical_formula: self
                .search_empirical_formula_widget
                .selected
                .first()
                .and_then(|s| s.id),
            entity: self
                .search_entity_widget
                .selected
                .first()
                .and_then(|s| s.id),
            hazard_statements: (!hazard_statements.is_empty()).then_some(hazard_statements),
            name: self.search_name_widget.selected.first().and_then(|s| s.id),
            precautionary_statements: (!precautionary_statements.is_empty())
                .then_some(precautionary_statements),
            producer_ref: self
                .search_producer_ref_widget
                .selected
                .first()
                .and_then(|s| s.id),
            signal_word: self
                .search_signal_word_widget
                .selected
                .first()
                .and_then(|s| s.id),
            storage_barecode: (!self.search_barecode.is_empty())
                .then(|| self.search_barecode.clone()),
            storage_to_destroy: self.search_product_to_destroy,
            store_location: self
                .search_store_location_widget
                .selected
                .first()
                .and_then(|s| s.id),
            symbols: (!symbols.is_empty()).then_some(symbols),
            tags: (!tags.is_empty()).then_some(tags),
            ..Default::default()
        };

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

        filter
    }

    pub fn get_connected_user(&self) -> Result<Option<Person>, String> {
        let connected_user_lock = match self.connected_user.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<Person> = (*connected_user_lock).clone();

        Ok(result)
    }

    pub fn get_pubchem_product(&self) -> Result<Option<PubchemProduct>, String> {
        let pubchem_product_lock = match self.pubchem_product.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<PubchemProduct> = (*pubchem_product_lock).clone();

        Ok(result)
    }

    pub fn get_pubchem_autocomplete(&self) -> Result<Option<Autocomplete>, String> {
        let pubchem_autocomplete_lock = match self.pubchem_autocomplete.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<Autocomplete> = (*pubchem_autocomplete_lock).clone();

        Ok(result)
    }

    pub fn get_store_locations_and_count(
        &self,
    ) -> Result<Option<(Vec<StoreLocation>, u64)>, String> {
        let store_locations_and_count_lock = match self.store_locations.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<(Vec<StoreLocation>, u64)> = (*store_locations_and_count_lock).clone();

        Ok(result)
    }

    pub fn get_people_and_count(&self) -> Result<Option<(Vec<Person>, u64)>, String> {
        let people_and_count_lock = match self.people.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<(Vec<Person>, u64)> = (*people_and_count_lock).clone();

        Ok(result)
    }

    pub fn get_entities_and_count(&self) -> Result<Option<(Vec<Entity>, u64)>, String> {
        let entities_and_count_lock = match self.entities.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<(Vec<Entity>, u64)> = (*entities_and_count_lock).clone();

        Ok(result)
    }

    pub fn get_products_and_count(&self) -> Result<Option<(Vec<Product>, u64)>, String> {
        let products_and_count_lock = match self.products.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<(Vec<Product>, u64)> = (*products_and_count_lock).clone();

        Ok(result)
    }

    pub fn get_storages_and_count(&self) -> Result<Option<(Vec<Storage>, u64)>, String> {
        let storages_and_count_lock = match self.storages.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<(Vec<Storage>, u64)> = (*storages_and_count_lock).clone();

        Ok(result)
    }

    pub fn get_export_storages(&self) -> Result<Option<String>, String> {
        let export_storages_lock = match self.export_storages.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<String> = (*export_storages_lock).clone();

        Ok(result)
    }

    // pub fn get_permissions(&self) -> Result<Option<Vec<Permission>>, String> {
    //     let permissions_lock = match self.permissions.lock() {
    //         Ok(locked) => locked,
    //         Err(e) => {
    //             elog!(error, e.to_string());
    //             return Err(e.to_string());
    //         }
    //     };

    //     let result: Option<Vec<Permission>> = (*permissions_lock).clone();

    //     Ok(result)
    // }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Update window size.
        self.state.window_rect = ui.max_rect();

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
            get_connected_user(
                Arc::clone(&self.connected_user),
                self.channel_sender.clone(),
            );

            // Get products.
            let mut request_filter = self.get_request_filter();
            request_filter.order_by = Some(self.products_order_by.to_string());
            request_filter.order = self.products_order.to_string();

            get_products(
                &request_filter,
                Arc::clone(&self.products),
                false,
                self.channel_sender.clone(),
            );
        });

        // Check channel messages.
        if let Some(channel_receiver) = self.channel_receiver.as_ref()
            && let Ok(maybe_message) = channel_receiver.recv(Some(Duration::ZERO))
            && let Some(message) = maybe_message
        {
            self.is_loading = message;
        }

        // Handle actions.
        while let Some(action) = self.state.action.pop_front() {
            match action {
                Action::GetProducts(append) => {
                    let mut request_filter = self.get_request_filter();
                    request_filter.order_by = Some(self.products_order_by.to_string());
                    request_filter.order = self.products_order.to_string();

                    get_products(
                        &request_filter,
                        Arc::clone(&self.products),
                        append,
                        self.channel_sender.clone(),
                    );
                }
                Action::GetStorages(append) => {
                    let mut request_filter = self.get_request_filter();
                    request_filter.order_by = Some(self.storages_order_by.to_string());
                    request_filter.order = self.storages_order.to_string();

                    if self.storages_show_archives {
                        request_filter.storage_archive = Some(true);
                    } else {
                        request_filter.storage_archive = Some(false);
                    }

                    get_storages(
                        &request_filter,
                        Arc::clone(&self.storages),
                        append,
                        self.channel_sender.clone(),
                    );
                }
                Action::GetStorelocations => {
                    get_store_locations(
                        &RequestFilter {
                            // limit: Some(SEARCH_LIMIT),
                            search: Some(self.search_store_location.clone()),
                            order: self.store_locations_order.to_string(),
                            order_by: Some(self.store_locations_order_by.to_string()),
                            ..Default::default()
                        },
                        Arc::clone(&self.store_locations),
                        false,
                        self.channel_sender.clone(),
                    );
                }
                Action::GetEntities => {
                    get_entities(
                        &RequestFilter {
                            // limit: Some(SEARCH_LIMIT),
                            search: Some(self.search_entity.clone()),
                            order: self.entities_order.to_string(),
                            // order_by: Some(self.store_locations_order_by.to_string()),
                            ..Default::default()
                        },
                        Arc::clone(&self.entities),
                        false,
                        self.channel_sender.clone(),
                    );
                }
                Action::None => {}
                Action::GetPubchemAutocomplete => {
                    get_pubchem_autocomplete(
                        &self.pubchem_search,
                        Arc::clone(&self.pubchem_autocomplete),
                        self.channel_sender.clone(),
                    );
                }
                Action::GetPubchemProduct => {
                    get_pubchem_product(
                        &self.pubchem_search_name_clicked,
                        Arc::clone(&self.pubchem_product),
                        self.channel_sender.clone(),
                    );
                }
                Action::GetPermissions => {
                    get_permissions(&Arc::clone(&self.permissions));
                }
                Action::ExportProducts => todo!(),
                Action::ExportStorages => {
                    let mut request_filter = self.get_request_filter();
                    request_filter.order_by = Some(self.storages_order_by.to_string());
                    request_filter.order = self.storages_order.to_string();

                    if self.storages_show_archives {
                        request_filter.storage_archive = Some(true);
                    } else {
                        request_filter.storage_archive = Some(false);
                    }

                    export_storages(
                        &request_filter,
                        Arc::clone(&self.export_storages),
                        self.channel_sender.clone(),
                    );
                }
                Action::GetPeople => {
                    get_people(
                        &RequestFilter {
                            // limit: Some(SEARCH_LIMIT),
                            search: Some(self.search_person.clone()),
                            order: self.people_order.to_string(),
                            // order_by: Some(self.store_locations_order_by.to_string()),
                            ..Default::default()
                        },
                        Arc::clone(&self.people),
                        false,
                        self.channel_sender.clone(),
                    );
                }
            }
        }

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

        self.create_product_tag_widget.check_loading();
        self.create_product_category_widget.check_loading();
        self.create_product_name_widget.check_loading();
        self.create_product_synonym_widget.check_loading();
        self.create_product_empirical_formula_widget.check_loading();
        self.create_product_linear_formula_widget.check_loading();

        // Check export storages ready to download.
        if let Ok(Some(export_storages)) = self.get_export_storages() {
            download_csv(&export_storages, "export_storages.csv");

            let mut export_storages_lock = self
                .export_storages
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            (*export_storages_lock) = None;
        }

        // Check channels for messages.
        // if let Some(receiver) = &self.receiver {
        //     receiver.try_recv(){
        //     }
        // }

        let ctx = ui.ctx();

        // Foreground loading overlay.
        if self.is_loading {
            let screen_rect = ctx.viewport_rect();

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

        match self.get_connected_user() {
            Ok(connected_user) => {
                if connected_user.is_some() {
                    main::ui::update(self, ui, frame);
                } else {
                    egui::Panel::top("wait_user_info")
                        .show(ui, |ui| ui.label(t!("wait_user_info")));
                }
            }
            Err(e) => log::error!("{e}"),
        }
    }
}

fn load_svg_texture(
    ctx: &egui::Context,
    name: &str,
    svg_bytes: &[u8],
) -> Result<egui::TextureHandle, Box<dyn Error>> {
    use resvg::tiny_skia;
    use usvg;

    let tree = usvg::Tree::from_data(svg_bytes, &usvg::Options::default())?;
    let size = tree.size().to_int_size();
    let mut pixmap =
        tiny_skia::Pixmap::new(size.width(), size.height()).ok_or("failed to create pixmap")?;

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let image = egui::ColorImage::from_rgba_unmultiplied(
        [size.width() as usize, size.height() as usize],
        pixmap.data(),
    );

    Ok(ctx.load_texture(name, image, egui::TextureOptions::LINEAR))
}

// fn load_texture(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
//     let image = image::load_from_memory(bytes).unwrap().to_rgba8();
//     let size = [image.width() as usize, image.height() as usize];
//     let pixels = image.into_raw();
//     let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

//     ctx.load_texture(name.to_string(), color_image, egui::TextureOptions::LINEAR)
// }

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

fn setup_custom_fonts(ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
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
        .ok_or("can not get font families")?;

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

    Ok(())
}
