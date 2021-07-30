use chrono::{prelude::*, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ops::Add;

use crate::config::Config;

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manager {
    secret: String,
}

impl Manager {
    pub fn new(cfg: &Config) -> Self {
        Manager {
            secret: cfg.secret_key_base.to_owned(),
        }
    }

    pub fn token_for_user(&self, user: &str) -> anyhow::Result<String> {
        let claims = Claims {
            sub: user.to_owned(),
            iss: "tinychat".to_owned(),
            iat: Utc::now().timestamp(),
            exp: Utc::now().add(Duration::days(360)).timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        return Ok(token);
    }

    pub fn username_from_token(&self, token: &str) -> anyhow::Result<String> {
        let token_contents = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        return Ok(token_contents.claims.sub);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let cfg = Config {
            secret_key_base: "secret".to_owned(),
        };
        let manager = Manager::new(&cfg);

        let user = "user".to_owned();
        let token = manager.token_for_user(&user).unwrap();
        let username = manager.username_from_token(&token).unwrap();

        assert_eq!(user, username);
    }
}
