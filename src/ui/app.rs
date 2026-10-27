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
use crate::api::versioninfo::get_version_info;
use crate::config::{
    APP_BOTTOM_MARGIN, APP_CORNER_RADIUS, APP_LEFT_MARGIN, APP_RIGHT_MARGIN, APP_TOP_MARGIN,
    CREATE_PRODUCT_FORM_CORNER_RADIUS, CREATE_PRODUCT_FORM_DESIRED_WIDTH,
    CREATE_PRODUCT_FORM_INNER_MARGIN, CREATE_PRODUCT_FORM_WIDGET_HORIZONTAL_SPACING,
    CREATE_PRODUCT_FORM_WIDGET_VERTICAL_SPACING, ERROR_COLOR, FAINT_BG_COLOR, HEADER_BOTTOM_MARGIN,
    HOVERED_BG_COLOR, HOVERED_STROKE, INFO_COLOR, INPUT_FILLED_STROKE, NORMAL_BG_COLOR,
    NORMAL_STROKE, PRODUCT_LABEL_ACTION_WIDTH, PRODUCT_LABEL_INNER_MARGIN,
    PRODUCT_LABEL_OUTER_MARGIN, PRODUCT_LABEL_PLUS_WIDTH, SEARCH_FORM_INNER_MARGIN,
    SEARCH_FORM_WIDGET_HORIZONTAL_SPACING, SEARCH_FORM_WIDGET_VERTICAL_SPACING, SEARCH_FORM_WIDTH,
    SELECT2_BORDER_WHEN_SELECTED_CORNER_RADIUS, SELECT2_BORDER_WHEN_SELECTED_MARGIN,
    SELECT2_BORDER_WHEN_SELECTED_STROKE, SELECTED_BG_COLOR, SELECTED_STROKE,
    STORAGE_LABEL_ACTION_WIDTH, STORAGE_LABEL_INNER_MARGIN, STORAGE_LABEL_OUTER_MARGIN,
    STORAGE_LABEL_PLUS_WIDTH, WEAK_TEXT_COLOR,
};
use crate::download::download_csv;
use crate::types::{
    Permission, PermissionStatus, ProductType, SharedPermissionList, SharedPerson,
    SharedVersionInfo,
};
use crate::ui::domain_state::{
    EntityState, PeopleState, ProductState, PubchemState, StorageState, StoreLocationState,
};
use crate::ui::forms::createproduct::CreateProductForm;
use crate::ui::forms::search::SearchForm;
use crate::ui::pages::main;
use crate::ui::state::Action;
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
use chimitheque_types::versioninfo::VersionInfo;
use eframe::CreationContext;
use egui::{CornerRadius, Id, Stroke, TextureHandle};
use elegance::{BuiltInTheme, Theme};
use rust_i18n::t;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::Once;
use std::time::Duration;
use wasm_rs_shared_channel::spsc::{Receiver, Sender, channel};

static START: Once = Once::new();

#[derive(Default)]
pub struct App {
    // Application state.
    pub state: ApplicationState,

    // Channel for communication between application and functions.
    pub channel_sender: Option<Sender<i32>>,
    pub channel_receiver: Option<Receiver<i32>>,

    // Image textures.
    pub textures: HashMap<String, TextureHandle>,

    // Search form.
    pub search_form: SearchForm,

    // Create product form.
    pub create_product_form: CreateProductForm,

    // Version info.
    pub version_info: SharedVersionInfo,
    // User information.
    pub connected_user: SharedPerson,
    // Permissions.
    pub permissions: SharedPermissionList,

    // Domain sub-structs.
    pub product_state: ProductState,
    pub storage_state: StorageState,
    pub store_location_state: StoreLocationState,
    pub entity_state: EntityState,
    pub people_state: PeopleState,
    pub pubchem_state: PubchemState,

    // Loading state.
    pub in_flight: i32,
}

impl App {
    /// # Panics
    ///
    /// Will panic if svg pictures cannot be loaded, or if custom fonts cannot be set.
    #[allow(clippy::unwrap_used)]
    #[must_use]
    pub fn new(cc: &CreationContext) -> Self {
        // Setup custom fonts.
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor_icons::add_fonts(&mut fonts);
        // make Phosphor available through the standard proportional family, this is required for egui icons to work
        let proportional = fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap();

        proportional.extend([
            "phosphor-icons".to_owned(),
            "phosphor-icons-bold".to_owned(),
            "phosphor-icons-fill".to_owned(),
            "phosphor-icons-light".to_owned(),
            "phosphor-icons-thin".to_owned(),
        ]);
        cc.egui_ctx.set_fonts(fonts);

        log::set_max_level(log::LevelFilter::Debug);

        // Create application channel.
        let channel_channel = channel::<i32>(1024);
        let (channel_sender, channel_receiver) = channel_channel.split();

        // Create application state and visual.
        let state = ApplicationState::new(&rust_i18n::locale());

        // Create search form.
        let search_form = SearchForm::new();

        // Create create product form.
        let create_product_form = CreateProductForm::new();

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

            search_form,
            create_product_form,

            channel_sender: Some(channel_sender),
            channel_receiver: Some(channel_receiver),
            textures,
            ..Default::default()
        }
    }

    /// Returns the in-flight counter sender.
    /// # Panics
    /// Panics if called before `App::new()` initialises the channel, which
    /// cannot happen in normal operation.
    fn sender(&self) -> Sender<i32> {
        self.channel_sender
            .clone()
            .unwrap_or_else(|| unreachable!("channel_sender is None — called before App::new()"))
    }

    // Check if the user has permission to perform an action on a given entity.
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
            .search_form
            .search_hazard_statement_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let precautionary_statements: Vec<_> = self
            .search_form
            .search_precautionary_statement_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let symbols: Vec<_> = self
            .search_form
            .search_symbol_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let tags: Vec<_> = self
            .search_form
            .search_tag_widget
            .selected
            .iter()
            .filter_map(|s| s.id)
            .collect();

        let mut filter = RequestFilter {
            offset: Some(self.search_form.search_offset),
            limit: Some(self.search_form.search_limit),
            borrowing: self.search_form.search_product_borrowed,
            cas_number: self
                .search_form
                .search_cas_number_widget
                .selected
                .first()
                .and_then(|s| s.id),
            is_cmr: self.search_form.search_product_cmr,
            category: self
                .search_form
                .search_category_widget
                .selected
                .first()
                .and_then(|s| s.id),
            custom_name_part_of: (!self.search_form.search_part_of_name.is_empty())
                .then(|| self.search_form.search_part_of_name.clone()),
            empirical_formula: self
                .search_form
                .search_empirical_formula_widget
                .selected
                .first()
                .and_then(|s| s.id),
            entity: self
                .search_form
                .search_entity_widget
                .selected
                .first()
                .and_then(|s| s.id),
            hazard_statements: (!hazard_statements.is_empty()).then_some(hazard_statements),
            name: self
                .search_form
                .search_name_widget
                .selected
                .first()
                .and_then(|s| s.id),
            precautionary_statements: (!precautionary_statements.is_empty())
                .then_some(precautionary_statements),
            producer_ref: self
                .search_form
                .search_producer_ref_widget
                .selected
                .first()
                .and_then(|s| s.id),
            signal_word: self
                .search_form
                .search_signal_word_widget
                .selected
                .first()
                .and_then(|s| s.id),
            storage_barecode: (!self.search_form.search_barecode.is_empty())
                .then(|| self.search_form.search_barecode.clone()),
            storage_to_destroy: self.search_form.search_product_to_destroy,
            store_location: self
                .search_form
                .search_store_location_widget
                .selected
                .first()
                .and_then(|s| s.id),
            symbols: (!symbols.is_empty()).then_some(symbols),
            tags: (!tags.is_empty()).then_some(tags),
            ..Default::default()
        };

        match self.search_form.search_product_type {
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

    /// Builds a `RequestFilter` for storage-related requests, applying the
    /// current sort order and archive-visibility preference.
    fn build_storages_filter(&self) -> RequestFilter {
        let mut filter = self.get_request_filter();
        filter.order_by = Some(self.storage_state.storages_order_by.to_string());
        filter.order = self.storage_state.storages_order.to_string();
        filter.storage_archive = Some(self.storage_state.storages_show_archives);
        filter
    }

    pub fn get_version_info(&self) -> Result<Option<VersionInfo>, String> {
        let version_info_lock = match self.version_info.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<VersionInfo> = (*version_info_lock).clone();

        Ok(result)
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
        let pubchem_product_lock = match self.pubchem_state.pubchem_product.lock() {
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
        let pubchem_autocomplete_lock = match self.pubchem_state.pubchem_autocomplete.lock() {
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
        let store_locations_and_count_lock = match self.store_location_state.store_locations.lock()
        {
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
        let people_and_count_lock = match self.people_state.people.lock() {
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
        let entities_and_count_lock = match self.entity_state.entities.lock() {
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
        let products_and_count_lock = match self.product_state.products.lock() {
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
        let storages_and_count_lock = match self.storage_state.storages.lock() {
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
        let export_storages_lock = match self.storage_state.export_storages.lock() {
            Ok(locked) => locked,
            Err(e) => {
                elog!(error, e.to_string());
                return Err(e.to_string());
            }
        };

        let result: Option<String> = (*export_storages_lock).clone();

        Ok(result)
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Update window size.
        self.state.window_rect = ui.max_rect();

        // Set zoom factor.
        let ctx = ui.ctx();
        ctx.set_zoom_factor(self.state.zoom_factor);

        // Do one time startup job.
        START.call_once(|| {
            // Init statics.
            let visual = &ui.visuals();
            let widgets = &visual.widgets;
            let selection = &visual.selection;

            HOVERED_BG_COLOR.get_or_init(|| visual.widgets.inactive.bg_fill);
            SELECTED_BG_COLOR.get_or_init(|| visual.selection.bg_fill);
            NORMAL_BG_COLOR.get_or_init(|| visual.window_fill);
            FAINT_BG_COLOR.get_or_init(|| visual.faint_bg_color);
            WEAK_TEXT_COLOR.get_or_init(|| visual.weak_text_color.unwrap_or_default());
            NORMAL_STROKE.get_or_init(|| widgets.noninteractive.bg_stroke);
            HOVERED_STROKE.get_or_init(|| widgets.hovered.fg_stroke);
            SELECTED_STROKE.get_or_init(|| selection.stroke);
            INFO_COLOR.get_or_init(|| egui::Color32::from_rgb(60, 180, 95));
            ERROR_COLOR.get_or_init(|| egui::Color32::from_rgb(220, 70, 70));
            APP_TOP_MARGIN.get_or_init(|| 20);
            APP_BOTTOM_MARGIN.get_or_init(|| 10);
            APP_LEFT_MARGIN.get_or_init(|| 50);
            APP_RIGHT_MARGIN.get_or_init(|| 50);
            HEADER_BOTTOM_MARGIN.get_or_init(|| 20);
            APP_CORNER_RADIUS.get_or_init(|| CornerRadius::same(5));
            PRODUCT_LABEL_INNER_MARGIN.get_or_init(|| egui::Margin::symmetric(20, 10));
            PRODUCT_LABEL_OUTER_MARGIN.get_or_init(|| egui::Margin::symmetric(50, 5));
            PRODUCT_LABEL_PLUS_WIDTH.get_or_init(|| 50.0);
            PRODUCT_LABEL_ACTION_WIDTH.get_or_init(|| 50.0);
            STORAGE_LABEL_INNER_MARGIN.get_or_init(|| egui::Margin::symmetric(20, 10));
            STORAGE_LABEL_OUTER_MARGIN.get_or_init(|| egui::Margin::symmetric(50, 5));
            STORAGE_LABEL_PLUS_WIDTH.get_or_init(|| 50.0);
            STORAGE_LABEL_ACTION_WIDTH.get_or_init(|| 50.0);
            SEARCH_FORM_WIDTH.get_or_init(|| 925.0);
            SEARCH_FORM_INNER_MARGIN.get_or_init(|| egui::Margin::symmetric(20, 20));
            SEARCH_FORM_WIDGET_HORIZONTAL_SPACING.get_or_init(|| 20.0);
            SEARCH_FORM_WIDGET_VERTICAL_SPACING.get_or_init(|| 20.0);
            CREATE_PRODUCT_FORM_DESIRED_WIDTH.get_or_init(|| 925.0);
            CREATE_PRODUCT_FORM_INNER_MARGIN.get_or_init(|| egui::Margin::symmetric(20, 20));
            CREATE_PRODUCT_FORM_CORNER_RADIUS.get_or_init(|| 5.0);
            CREATE_PRODUCT_FORM_WIDGET_HORIZONTAL_SPACING.get_or_init(|| 20.0);
            CREATE_PRODUCT_FORM_WIDGET_VERTICAL_SPACING.get_or_init(|| 20.0);
            SELECT2_BORDER_WHEN_SELECTED_STROKE.get_or_init(|| Stroke {
                width: 4.0,
                color: selection.bg_fill,
            });
            SELECT2_BORDER_WHEN_SELECTED_MARGIN.get_or_init(|| egui::Margin::symmetric(5, 5));
            SELECT2_BORDER_WHEN_SELECTED_CORNER_RADIUS.get_or_init(|| 5.0);
            INPUT_FILLED_STROKE.get_or_init(|| Stroke {
                width: 4.0,
                color: selection.bg_fill,
            });

            // Set forms layout.
            self.search_form.apply_layout();
            self.create_product_form.apply_layout();

            // Get connected user.
            get_connected_user(Arc::clone(&self.connected_user), self.sender());

            // Get products.
            let mut request_filter = self.get_request_filter();
            request_filter.order_by = Some(self.product_state.products_order_by.to_string());
            request_filter.order = self.product_state.products_order.to_string();

            get_products(
                &request_filter,
                Arc::clone(&self.product_state.products),
                false,
                self.sender(),
            );

            // Get version info.
            get_version_info(Arc::clone(&self.version_info), self.sender());
        });

        // Drain all pending loading-counter messages.
        if let Some(channel_receiver) = self.channel_receiver.as_ref() {
            while let Ok(Some(delta)) = channel_receiver.recv(Some(Duration::ZERO)) {
                self.in_flight += delta;
                if self.in_flight < 0 {
                    self.in_flight = 0;
                }
            }
        }
        if self.in_flight == 0 {
            let ctx = ui.ctx();
            ctx.request_repaint();
        }

        // Handle actions.
        while let Some(action) = self.state.action.pop_front() {
            match action {
                Action::GetProducts(append) => {
                    let mut request_filter = self.get_request_filter();
                    request_filter.order_by =
                        Some(self.product_state.products_order_by.to_string());
                    request_filter.order = self.product_state.products_order.to_string();

                    get_products(
                        &request_filter,
                        Arc::clone(&self.product_state.products),
                        append,
                        self.sender(),
                    );
                }
                Action::GetStorages(append) => {
                    let request_filter = self.build_storages_filter();

                    get_storages(
                        &request_filter,
                        Arc::clone(&self.storage_state.storages),
                        append,
                        self.sender(),
                    );
                }
                Action::GetStorelocations => {
                    get_store_locations(
                        &RequestFilter {
                            // limit: Some(SEARCH_LIMIT),
                            search: Some(self.store_location_state.search_store_location.clone()),
                            order: self.store_location_state.store_locations_order.to_string(),
                            order_by: Some(
                                self.store_location_state
                                    .store_locations_order_by
                                    .to_string(),
                            ),
                            ..Default::default()
                        },
                        Arc::clone(&self.store_location_state.store_locations),
                        false,
                        self.sender(),
                    );
                }
                Action::GetEntities => {
                    get_entities(
                        &RequestFilter {
                            // limit: Some(SEARCH_LIMIT),
                            search: Some(self.entity_state.search_entity.clone()),
                            order: self.entity_state.entities_order.to_string(),
                            // order_by: Some(self.store_locations_order_by.to_string()),
                            ..Default::default()
                        },
                        Arc::clone(&self.entity_state.entities),
                        false,
                        self.sender(),
                    );
                }
                Action::None => {}
                Action::GetPubchemAutocomplete => {
                    get_pubchem_autocomplete(
                        &self.pubchem_state.pubchem_search,
                        Arc::clone(&self.pubchem_state.pubchem_autocomplete),
                        self.sender(),
                    );
                }
                Action::GetPubchemProduct => {
                    get_pubchem_product(
                        &self.pubchem_state.pubchem_search_name_clicked,
                        Arc::clone(&self.pubchem_state.pubchem_product),
                        self.sender(),
                    );
                }
                Action::GetPermissions => {
                    get_permissions(&Arc::clone(&self.permissions));
                }
                Action::ExportProducts => todo!(),
                Action::ExportStorages => {
                    let request_filter = self.build_storages_filter();

                    export_storages(
                        &request_filter,
                        Arc::clone(&self.storage_state.export_storages),
                        self.sender(),
                    );
                }
                Action::GetPeople => {
                    get_people(
                        &RequestFilter {
                            // limit: Some(SEARCH_LIMIT),
                            search: Some(self.people_state.search_person.clone()),
                            order: self.people_state.people_order.to_string(),
                            // order_by: Some(self.store_locations_order_by.to_string()),
                            ..Default::default()
                        },
                        Arc::clone(&self.people_state.people),
                        false,
                        self.sender(),
                    );
                }
            }
        }

        // Check loading state of select2 widgets.
        self.search_form
            .search_store_location_widget
            .check_loading();
        self.search_form.search_name_widget.check_loading();
        self.search_form.search_entity_widget.check_loading();
        self.search_form.search_category_widget.check_loading();
        self.search_form
            .search_empirical_formula_widget
            .check_loading();
        self.search_form
            .search_hazard_statement_widget
            .check_loading();
        self.search_form
            .search_precautionary_statement_widget
            .check_loading();
        self.search_form.search_symbol_widget.check_loading();
        self.search_form.search_tag_widget.check_loading();
        self.search_form.search_cas_number_widget.check_loading();
        self.search_form.search_producer_ref_widget.check_loading();
        self.search_form.search_signal_word_widget.check_loading();

        self.create_product_form
            .create_product_tag_widget
            .check_loading();
        self.create_product_form
            .create_product_category_widget
            .check_loading();
        self.create_product_form
            .create_product_name_widget
            .check_loading();
        self.create_product_form
            .create_product_synonym_widget
            .check_loading();
        self.create_product_form
            .create_product_empirical_formula_widget
            .check_loading();
        self.create_product_form
            .create_product_linear_formula_widget
            .check_loading();
        self.create_product_form
            .create_product_cas_number_widget
            .check_loading();
        self.create_product_form
            .create_product_ce_number_widget
            .check_loading();
        self.create_product_form
            .create_product_unit_molecular_weight_widget
            .check_loading();
        self.create_product_form
            .create_product_unit_temperature_widget
            .check_loading();
        self.create_product_form
            .create_product_physical_state_widget
            .check_loading();
        self.create_product_form
            .create_product_class_of_compound_widget
            .check_loading();
        self.create_product_form
            .create_product_signal_word_widget
            .check_loading();
        self.create_product_form
            .create_product_symbol_widget
            .check_loading();
        self.create_product_form
            .create_product_hazard_statement_widget
            .check_loading();
        self.create_product_form
            .create_product_precautionary_statement_widget
            .check_loading();

        // Check export storages ready to download.
        if let Ok(Some(export_storages)) = self.get_export_storages() {
            download_csv(&export_storages, "export_storages.csv");

            let mut export_storages_lock = self
                .storage_state
                .export_storages
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            (*export_storages_lock) = None;
        }

        let ctx = ui.ctx();

        // Foreground loading overlay.
        if self.in_flight > 0 {
            let screen_rect = ctx.viewport_rect();

            // keep repainting so spinner animates
            ctx.request_repaint_after(std::time::Duration::from_millis(16));

            egui::Area::new(Id::unique("spinner_overlay"))
                .order(egui::Order::Foreground)
                .fixed_pos(screen_rect.center() - egui::vec2(20.0, 20.0))
                .interactable(false)
                .show(ctx, |ui| {
                    ui.add(egui::Spinner::new().size(40.0));
                });
        }

        // Update connected user.
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

// Loads an SVG from bytes and converts it into an egui texture.
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
