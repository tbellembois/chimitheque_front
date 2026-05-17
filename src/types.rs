use chimitheque_types::person::Person;
use chimitheque_types::pubchem::Autocomplete;
use chimitheque_types::pubchemproduct::PubchemProduct;
use chimitheque_types::{entity::Entity, product::Product, storelocation::StoreLocation};
use std::sync::{Arc, Mutex};

pub type SharedStoreLocationAndCountList = Arc<Mutex<Option<(Vec<StoreLocation>, u64)>>>;
pub type SharedEntityAndCountList = Arc<Mutex<Option<(Vec<Entity>, u64)>>>;
pub type SharedProductAndCountList = Arc<Mutex<Option<(Vec<Product>, u64)>>>;
pub type SharedPubchemAutocomplete = Arc<Mutex<Option<Autocomplete>>>;
pub type SharedPubchemProduct = Arc<Mutex<Option<PubchemProduct>>>;
pub type SharedPerson = Arc<Mutex<Option<Person>>>;
