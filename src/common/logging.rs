use lazy_static::lazy_static;
use serde::Deserialize;
use slog::{self, Drain};
use std::sync::{Mutex, Once};

pub use slog::{debug, error, info, trace, warn};

lazy_static! {
    static ref LOGGER: Mutex<Option<slog::Logger>> = Mutex::new(None);

    /// Initializes the global logger once.
    static ref INIT_GLOBAL_LOGGER: Once = Once::new();

    /// Prevents the global logger from being dropped.
    static ref GLOBAL_LOGGER_SCOPE_GUARD: Mutex<Option<slog_scope::GlobalLoggerGuard>> = Mutex::new(None);
}

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Log level.
    pub level: String,
    /// Log format.
    pub format: String,
    /// Log file.
    pub file: Option<String>,
}

pub type Logger = slog::Logger;

/// Start initializes the logger.
pub fn start(cfg: Config) {
    let format_drain = match cfg.format.as_str() {
        "json" => {
            let drain = slog_json::Json::default(std::io::stderr()).fuse();
            slog_async::Async::default(drain)
        }
        _ => {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            slog_async::Async::default(drain)
        }
    };
    let level_drain = {
        let level = match cfg.level.as_str() {
            "trace" => slog::Level::Trace,
            "debug" => slog::Level::Debug,
            "info" => slog::Level::Info,
            "warning" => slog::Level::Warning,
            "error" => slog::Level::Error,
            _ => slog::Level::Info,
        };
        slog::LevelFilter::new(format_drain, level).fuse()
    };

    LOGGER
        .lock()
        .unwrap()
        .get_or_insert(slog::Logger::root(level_drain, slog::o!()));

    INIT_GLOBAL_LOGGER.call_once(|| {
        let global_logger = get_logger("global");
        GLOBAL_LOGGER_SCOPE_GUARD
            .lock()
            .unwrap()
            .get_or_insert(slog_scope::set_global_logger(global_logger));
        slog_stdlog::init().unwrap();
    });

    let logger = get_logger("logging");
    info!(logger, "Logger started");
}

/// `start` must be called before `get_logger`.
pub fn get_logger(module: &'static str) -> slog::Logger {
    LOGGER
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .new(slog::o!("module" => module))
}

/// Must be called when using std::process::exit so that the async drain
/// can properly flush.
pub fn stop() {
    LOGGER.lock().unwrap().take();
}
