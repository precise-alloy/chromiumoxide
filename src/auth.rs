use std::fmt;

/// Credentials for authentications
#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_debug_redacts_password() {
        let creds = Credentials {
            username: "alice".to_string(),
            password: "super-secret-password".to_string(),
        };
        let debug = format!("{creds:?}");
        assert!(debug.contains("alice"));
        assert!(!debug.contains("super-secret-password"));
        assert!(debug.contains("[REDACTED]"));
    }
}
