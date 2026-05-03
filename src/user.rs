use std::collections::HashMap;

use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub id: i64,
    pw_hash: Vec<u8>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}

#[derive(Clone)]
pub struct Backend {
    users: HashMap<i64, User>,
}

impl Default for Backend {
    fn default() -> Self {
        let users = [(
            13,
            User {
                username: "thomas".to_string(),
                id: 13,
                pw_hash: vec![],
            },
        )];
        Self {
            users: users.into(),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let maybe_user = self
            .users
            .values()
            .find(|user| user.username == creds.username);
        if let Some(ref user) = maybe_user {
            info!("[{}]({}) - authenticated", user.username, user.id);
        }
        Ok(maybe_user.cloned())
        // Ok(self.users.get(&username).cloned())
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(user_id).cloned())
    }
}
