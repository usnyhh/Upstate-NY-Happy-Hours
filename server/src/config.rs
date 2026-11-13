//! Runtime configuration, read from environment variables with local-dev defaults.

use std::net::{AddrParseError, SocketAddr};

/// `mode=rwc` creates the SQLite file if it doesn't exist.
const DEFAULT_DATABASE_URL: &str = "sqlite://happy_hours.db?mode=rwc";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:3000";

pub struct Config {
    /// `DATABASE_URL`
    pub database_url: String,
    /// `BIND_ADDR`
    pub bind_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, AddrParseError> {
        Ok(Self {
            database_url: env_or("DATABASE_URL", DEFAULT_DATABASE_URL),
            bind_addr: env_or("BIND_ADDR", DEFAULT_BIND_ADDR).parse()?,
        })
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.into())
}
