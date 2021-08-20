use rand::{self, distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Default, Eq, Serialize, Deserialize, PartialEq, Clone, Hash)]
pub struct Config {
    pub secret_key_base: String,
}

impl Config {
    pub fn from_env() -> Self {
        let secret: String = env::var("AUTH_SECRET_BASE").unwrap_or_else(|_| {
            log::warn!(
                "AUTH_SECRET_BASE not set, using a random string. This will break after a restart!"
            );

            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(|u| u as char)
                .collect::<String>()
        });

        Config {
            secret_key_base: secret,
        }
    }
}
