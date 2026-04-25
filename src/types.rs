use chimitheque_types::{name::Name, product::Product, storelocation::StoreLocation};
use std::sync::{Arc, Mutex};

pub type SharedStoreLocationList = Arc<Mutex<Option<(Vec<StoreLocation>, u64)>>>;
pub type SharedNameList = Arc<Mutex<Option<(Vec<Name>, u64)>>>;
pub type SharedProductList = Arc<Mutex<Option<(Vec<Product>, u64)>>>;
pub type SharedString = Arc<Mutex<Option<String>>>;
