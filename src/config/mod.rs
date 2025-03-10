use config::{Config, File};
use tokio::sync::RwLock;
use std::sync::Arc;
use lazy_static::lazy_static;

lazy_static! {
    static ref INSTANCE: Arc<RwLock<Config>> = {
        let mut config = Config::new();
        config.merge(File::with_name("Cargo.toml")).unwrap();
        Arc::new(RwLock::new(config))
    };
}

pub struct ConfigSingleton;

impl ConfigSingleton {
    pub fn get_instance() -> Arc<RwLock<Config>> {
        INSTANCE.clone()
    }
}

#[derive(Debug)]
pub enum ServerConfig {
    Address,
    Port,
}

impl ServerConfig {
    pub fn as_str(&self) -> &'static str {
        match self {
            ServerConfig::Address => "server.address",
            ServerConfig::Port => "server.port",
        }
    }
}
