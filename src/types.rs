use chimitheque_types::permission::PermissionItem;
use chimitheque_types::person::Person;
use chimitheque_types::pubchem::Autocomplete;
use chimitheque_types::pubchemproduct::PubchemProduct;
use chimitheque_types::storage::Storage;
use chimitheque_types::{entity::Entity, product::Product, storelocation::StoreLocation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::{Arc, Mutex};

pub type SharedString = Arc<Mutex<Option<String>>>;
pub type SharedStoreLocationAndCountList = Arc<Mutex<Option<(Vec<StoreLocation>, u64)>>>;
pub type SharedEntityAndCountList = Arc<Mutex<Option<(Vec<Entity>, u64)>>>;
pub type SharedProductAndCountList = Arc<Mutex<Option<(Vec<Product>, u64)>>>;
pub type SharedStorageAndCountList = Arc<Mutex<Option<(Vec<Storage>, u64)>>>;
pub type SharedPubchemAutocomplete = Arc<Mutex<Option<Autocomplete>>>;
pub type SharedPubchemProduct = Arc<Mutex<Option<PubchemProduct>>>;
pub type SharedPerson = Arc<Mutex<Option<Person>>>;
pub type SharedPermissionList = Arc<Mutex<Vec<Permission>>>;
pub type SharedPersonAndCountList = Arc<Mutex<Option<(Vec<Person>, u64)>>>;

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

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum StoragesOrderBy {
    #[default]
    Product,
    BatchNumber,
    StoreLocation,
    ModificationDate,
}

impl Display for StoragesOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoragesOrderBy::Product => write!(f, "product"),
            StoragesOrderBy::BatchNumber => write!(f, "batch_number"),
            StoragesOrderBy::StoreLocation => write!(f, "store_location"),
            StoragesOrderBy::ModificationDate => write!(f, "modification_date"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum ProductsOrderBy {
    #[default]
    Name,
    CasNumber,
    EmpiricalFormula,
}

impl Display for ProductsOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductsOrderBy::Name => write!(f, "name"),
            ProductsOrderBy::CasNumber => write!(f, "cas_number"),
            ProductsOrderBy::EmpiricalFormula => write!(f, "empirical_formula"),
        }
    }
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

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum GenericOrder {
    #[default]
    Asc,
    Desc,
}

impl Display for GenericOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericOrder::Asc => write!(f, "asc"),
            GenericOrder::Desc => write!(f, "desc"),
        }
    }
}
