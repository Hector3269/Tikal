use chrono::{DateTime, Duration, Utc};
use colored::*;
use std::collections::HashMap;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use super::LogLevel;

pub struct Logger;

impl Logger {
    pub fn init(level: LogLevel) {
        let filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.as_str()));

        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_level(true)
                    .with_thread_ids(true)
                    .with_line_number(true)
                    .with_thread_names(true)
                    .compact()
                    .with_ansi(true),
            )
            .init();
    }

    pub fn init_colored(level: LogLevel) {
        let filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.as_str()));

        let format = fmt::format()
            .with_level(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_line_number(true)
            .compact();

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().event_format(format).with_ansi(true))
            .init();
    }
}
