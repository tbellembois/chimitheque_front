use chimitheque_types::{entity::Entity, product::Product, storelocation::StoreLocation};
use std::sync::{Arc, Mutex};

pub type SharedStoreLocationAndCountList = Arc<Mutex<Option<(Vec<StoreLocation>, u64)>>>;
pub type SharedEntityAndCountList = Arc<Mutex<Option<(Vec<Entity>, u64)>>>;
pub type SharedProductAndCountList = Arc<Mutex<Option<(Vec<Product>, u64)>>>;
pub type SharedString = Arc<Mutex<Option<String>>>;
