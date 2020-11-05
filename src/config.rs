pub use config::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

pub const KEY_LENGTH: usize = 32;

type SecretKey = [u8; KEY_LENGTH];

fn default_expiration_seconds() -> i64 {
    24 * 3600
}

#[derive(Deserialize)]
pub struct Cookies {
    pub secret_key: u8,
    pub expiration_seconds: i64,
}

#[derive(Deserialize)]
pub struct DomainConfig {
    pub url: String,
}


fn default_key() -> String {
    "0123".repeat(8).into()
}

#[derive(Deserialize, Clone)]
pub struct MailConfig {
    pub driver: String,
    pub host: String,
    pub port: String,
    pub username: String,
    pub sender: String,
    pub encryption: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Clone)]
pub struct Config {



    // pub domain_cnf: DomainConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
