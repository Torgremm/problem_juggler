use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::user_repo::UserRow;

const HASH_KEY: &str = "dont-do-this-but-this-is-just-for-studies-anyway-so";

pub struct Token {
    user_id: i64,
    issued_at: u64,
    expires_at: u64,
    signature: String,
}

impl Token {
    fn new(user: &UserRow) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expiry = now + 3600;
        let payload = format!("{}|{}|{}", user.id, now, expiry);
        let mut hasher = DefaultHasher::new();
        payload.hash(&mut hasher);
        HASH_KEY.hash(&mut hasher);
        let sig = hasher.finish().to_string();

        Self {
            user_id: user.id,
            issued_at: now,
            expires_at: expiry,
            signature: sig,
        }
    }

    pub fn get(user: &UserRow) -> String {
        Self::new(user).encode()
    }

    fn encode(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.user_id, self.issued_at, self.expires_at, self.signature
        )
    }

    pub fn validate(token: String) -> bool {
        Self::decode(token).is_some()
    }

    fn decode(token: String) -> Option<Self> {
        let mut parts = token.split('|');
        let user_id: i64 = parts.next()?.parse().ok()?;
        let issued_at: u64 = parts.next()?.parse().ok()?;
        let expires_at: u64 = parts.next()?.parse().ok()?;
        let signature = parts.next()?.to_string();

        let payload = format!("{}|{}|{}", user_id, issued_at, expires_at);

        let mut hasher = DefaultHasher::new();
        payload.hash(&mut hasher);
        HASH_KEY.hash(&mut hasher);

        let expected_sig = hasher.finish().to_string();

        if expected_sig != signature {
            return None;
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        if now > expires_at {
            return None;
        }

        Some(Self {
            user_id,
            issued_at,
            expires_at,
            signature,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_token_and_validate() {
        let u = UserRow::test_row(1);
        let token = Token::get(&u);
        assert!(Token::validate(token));
        assert!(!Token::validate(
            "this should not validate I think".to_string()
        ))
    }
}
