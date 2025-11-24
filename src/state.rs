use std::sync::{Arc, RwLock};
use crate::{key_manager::KeyManager, models::config::Config};

pub struct State {
    pub config: Arc<RwLock<Config>>,
    pub key_manager: Arc<RwLock<KeyManager>>,
}

impl State {
    pub async fn new(config: Config, km: KeyManager) -> Self {
        Self{ config: Arc::new(RwLock::new(config)), key_manager: Arc::new(RwLock::new(km)) }
    }
}

impl Clone for State
{
    fn clone(&self) -> Self {
        State {
            config: self.config.clone(),
            key_manager: self.key_manager.clone(),
        }
    }
}
