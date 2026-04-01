use crate::error::apperror::AppError;
use chimitheque_types::{product::Product, requestfilter::RequestFilter};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ToApp {
    pub message: ToAppMessage,
}

pub struct ToWorker {
    pub message: ToWorkerMessage,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ToAppMessage {
    Pong,
    Error(AppError),
}

#[allow(dead_code)]
pub enum ToWorkerMessage {
    Ping,
    GetProducts(RequestFilter, Arc<Mutex<Option<(Vec<Product>, u64)>>>),
}
