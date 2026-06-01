use std::{collections::VecDeque, sync::Mutex};

pub static LOGS: std::sync::LazyLock<Mutex<VecDeque<LogMessage>>> =
    std::sync::LazyLock::new(|| Mutex::new(VecDeque::new()));

#[derive(Clone)]
pub enum LogMessage {
    Debug(String),
    Info(String),
    Error(String),
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogMessage::Info(msg) => {
                write!(f, "[INFO] {msg}")
            }
            LogMessage::Error(msg) => {
                write!(f, "[ERROR] {msg}")
            }
            LogMessage::Debug(msg) => {
                write!(f, "[DEBUG] {msg}")
            }
        }
    }
}

pub fn log(msg: LogMessage) {
    #[cfg(not(target_arch = "wasm32"))]
    match msg {
        LogMessage::Debug(ref msg) => log::debug!("DEBUG: {msg}"),
        LogMessage::Info(ref msg) => log::info!("INFO: {msg}"),
        LogMessage::Error(ref msg) => log::error!("ERROR: {msg}"),
    };

    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&msg.to_string().into());

    let mut logs = LOGS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    logs.push_back(msg);

    if logs.len() > 1000 {
        logs.pop_front();
    }
}

// #[macro_export]
// macro_rules! elog {
//     ($msg:expr) => {
//         $crate::logger::log($msg)
//     };

//     ($fmt:expr, $($arg:tt)*) => {
//         $crate::logger::log(format!($fmt, $($arg)*))
//     };
// }

#[macro_export]
macro_rules! elog {
    (debug, $msg:expr) => {
        $crate::logger::log(
            $crate::logger::LogMessage::Debug($msg.into())
        )
    };

    (debug, $fmt:expr, $($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogMessage::Debug(
                format!($fmt, $($arg)*)
            )
        )
    };

    (info, $msg:expr) => {
        $crate::logger::log(
            $crate::logger::LogMessage::Info($msg.into())
        )
    };

    (info, $fmt:expr, $($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogMessage::Info(
                format!($fmt, $($arg)*)
            )
        )
    };

    (error, $msg:expr) => {
        $crate::logger::log(
            $crate::logger::LogMessage::Error($msg.into())
        )
    };

    (error, $fmt:expr, $($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogMessage::Error(
                format!($fmt, $($arg)*)
            )
        )
    };
}
