use super::message::{ToApp, ToWorker};
use crate::{
    error::apperror::AppError,
    worker::message::{ToAppMessage, ToWorkerMessage},
};
use chimitheque_types::product::Product;
use eframe::egui::Context;
use log::{error, info};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread, time,
};

fn parse_retrieve_products_response(
    response: ehttp::Response,
) -> Result<(Vec<Product>, u64), AppError> {
    match response.status {
        200 => match response.text() {
            Some(text_response) => match serde_json::from_str(text_response) {
                Ok(json_response) => Ok(json_response),
                Err(e) => Err(AppError::InternalError(e.to_string())),
            },
            None => Err(AppError::UnexpectedEmptyResponse),
        },
        _ => match response.text() {
            Some(text_response) => Err(AppError::NotOkHTTPResponse(text_response.to_string())),
            None => Err(AppError::NotOkHTTPResponse(response.status.to_string())),
        },
    }
}

#[allow(dead_code)]
pub struct Worker {
    sender: Sender<ToApp>,
    receiver: Receiver<ToWorker>,
    egui_ctx: Context,
}

impl Worker {
    #[allow(dead_code)]
    pub fn new(sender: Sender<ToApp>, receiver: Receiver<ToWorker>, egui_ctx: Context) -> Self {
        Self {
            sender,
            receiver,
            egui_ctx,
        }
    }

    #[allow(dead_code)]
    pub fn init(&mut self) {
        info!("Worker starting up.");

        // Wait for <ToWorker> messages giving work to do.
        // Can send back <ToApp> messages to the GUI.
        // FIXME:
        // In case of a send error we can not "send" an AppError
        // to the app. We just log it. To be improved.
        loop {
            let maybe_message = self.receiver.recv();

            match maybe_message {
                Ok(message) => match message.message {
                    ToWorkerMessage::Ping => {
                        thread::sleep(time::Duration::from_secs(2));

                        if self
                            .sender
                            .send(ToApp {
                                message: ToAppMessage::Pong,
                            })
                            .is_err()
                        {
                            error!("failed to send ToAppMessage::Pong");
                        }
                    }
                    ToWorkerMessage::GetProducts(request_filter, products) => {
                        let request = ehttp::Request::get(format!(
                            "https://localhost:8443/back/products{}",
                            request_filter
                        ));

                        // let cloned_products = Arc::clone(&products);

                        ehttp::fetch(request, move |response| {
                            let mut locked_mutex = products.lock().unwrap();

                            let response_products =
                                parse_retrieve_products_response(response.unwrap()).unwrap();

                            *locked_mutex = Some(response_products);
                        });
                    }
                },
                Err(e) => {
                    if self
                        .sender
                        .send(ToApp {
                            message: ToAppMessage::Error(AppError::ChannelReceiveError),
                        })
                        .is_err()
                    {
                        // error!("failed to send ToAppMessage::Error for {}", e);
                    };
                }
            }
        }
    }
}
