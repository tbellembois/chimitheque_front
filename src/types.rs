use chimitheque_types::permission::PermissionItem;
use chimitheque_types::person::Person;
use chimitheque_types::pubchem::Autocomplete;
use chimitheque_types::pubchemproduct::PubchemProduct;
use chimitheque_types::storage::Storage;
use chimitheque_types::{entity::Entity, product::Product, storelocation::StoreLocation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::{Arc, Mutex};

pub type SharedStoreLocationAndCountList = Arc<Mutex<Option<(Vec<StoreLocation>, u64)>>>;
pub type SharedEntityAndCountList = Arc<Mutex<Option<(Vec<Entity>, u64)>>>;
pub type SharedProductAndCountList = Arc<Mutex<Option<(Vec<Product>, u64)>>>;
pub type SharedStorageAndCountList = Arc<Mutex<Option<(Vec<Storage>, u64)>>>;
pub type SharedPubchemAutocomplete = Arc<Mutex<Option<Autocomplete>>>;
pub type SharedPubchemProduct = Arc<Mutex<Option<PubchemProduct>>>;
pub type SharedPerson = Arc<Mutex<Option<Person>>>;
pub type SharedPermissionList = Arc<Mutex<Vec<Permission>>>;

#[derive(Clone, PartialEq, Default)]
pub enum PermissionStatus {
    #[default]
    ToRetrieve,
    InProgress,
    Done,
}

#[derive(Clone)]
pub struct Permission {
    pub unique_id: usize,
    pub status: PermissionStatus,
    pub item: PermissionItem,
    pub entity: Option<u64>,
    pub http_method: ehttp::Method,
    pub granted: bool,
}

#[derive(Default, PartialEq)]
pub enum ProductType {
    Chemical,
    Biological,
    Consumable,
    #[default]
    All,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum StoreLocationsOrderBy {
    #[default]
    Name,
    Entity,
    Parent,
}

impl Display for StoreLocationsOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreLocationsOrderBy::Name => write!(f, "store_location_name"),
            StoreLocationsOrderBy::Entity => write!(f, "entity.entity_name"),
            StoreLocationsOrderBy::Parent => write!(f, "store_location.store_location_name"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum StoreLocationsOrder {
    #[default]
    Asc,
    Desc,
}

impl Display for StoreLocationsOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreLocationsOrder::Asc => write!(f, "asc"),
            StoreLocationsOrder::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum EntitiesOrder {
    #[default]
    Asc,
    Desc,
}

impl Display for EntitiesOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntitiesOrder::Asc => write!(f, "asc"),
            EntitiesOrder::Desc => write!(f, "desc"),
        }
    }
}
