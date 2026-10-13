use crate::types::{
    GenericOrder, ProductsOrderBy, SharedEntityAndCountList, SharedPersonAndCountList,
    SharedProductAndCountList, SharedPubchemAutocomplete, SharedPubchemProduct,
    SharedStorageAndCountList, SharedStoreLocationAndCountList, SharedString, StoragesOrderBy,
    StoreLocationsOrderBy,
};

/// State for the product list page.
#[derive(Default)]
pub struct ProductState {
    pub products: SharedProductAndCountList,
    pub products_order_by: ProductsOrderBy,
    pub products_order: GenericOrder,
    pub product_cards_shown: Vec<u64>,
    pub product_cards_actions_shown: Vec<u64>,
}

/// State for the storage list page.
#[derive(Default)]
pub struct StorageState {
    pub storages: SharedStorageAndCountList,
    pub storages_order_by: StoragesOrderBy,
    pub storages_order: GenericOrder,
    pub storages_show_archives: bool,
    pub storage_cards_shown: Vec<u64>,
    pub storage_cards_actions_shown: Vec<u64>,
    pub export_storages: SharedString,
}

/// State for the store-location list page.
#[derive(Default)]
pub struct StoreLocationState {
    pub store_locations: SharedStoreLocationAndCountList,
    pub store_locations_order_by: StoreLocationsOrderBy,
    pub store_locations_order: GenericOrder,
    pub search_store_location: String,
    pub search_store_location_last_edit: f64,
    pub search_store_location_action_triggered: bool,
}

/// State for the entity list page.
#[derive(Default)]
pub struct EntityState {
    pub entities: SharedEntityAndCountList,
    pub entities_order: GenericOrder,
    pub search_entity: String,
    pub search_entity_last_edit: f64,
    pub search_entity_action_triggered: bool,
}

/// State for the people list page.
#[derive(Default)]
pub struct PeopleState {
    pub people: SharedPersonAndCountList,
    pub people_order: GenericOrder,
    pub search_person: String,
    pub search_person_last_edit: f64,
    pub search_person_action_triggered: bool,
}

/// State for the `PubChem` search page.
#[derive(Default)]
pub struct PubchemState {
    pub pubchem_autocomplete: SharedPubchemAutocomplete,
    pub pubchem_product: SharedPubchemProduct,
    pub pubchem_search: String,
    pub pubchem_search_name_clicked: String,
    pub pubchem_results_expanded: bool,
}
