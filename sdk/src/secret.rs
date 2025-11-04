/// Wrapper for password values that keeps the inner string private.
#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Password {
    secret: String,
}

impl Password {
    /// Creates a new password wrapper from a string-like value.
    pub fn new<S: Into<String>>(secret: S) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    /// Returns the inner secret as a string slice.
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// Consumes the wrapper and returns the secret string.
    pub fn into_secret(self) -> String {
        self.secret
    }
}

impl From<String> for Password {
    fn from(secret: String) -> Self {
        Self::new(secret)
    }
}

impl From<&str> for Password {
    fn from(secret: &str) -> Self {
        Self::new(secret)
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Password").field("secret", &"***").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_secret() {
        let password = Password::new("super-secret");

        assert_eq!(password.secret(), "super-secret");
    }

    #[test]
    fn into_secret_consumes_password() {
        let password = Password::new("super-secret");

        let secret = password.into_secret();

        assert_eq!(secret, "super-secret");
    }

    #[test]
    fn from_str_constructs_password() {
        let password: Password = "super-secret".into();

        assert_eq!(password.secret(), "super-secret");
    }

    #[test]
    fn debug_masks_secret() {
        let password = Password::new("super-secret");
        let debug_output = format!("{password:?}");

        assert_eq!(debug_output, "Password { secret: \"***\" }");
        assert!(!debug_output.contains("super-secret"));
    }

    #[test]
    fn serde_round_trip_preserves_secret() -> Result<(), serde_json::Error> {
        let password = Password::new("super-secret");

        let json = serde_json::to_string(&password)?;
        assert_eq!(json, "\"super-secret\"");

        let deserialized: Password = serde_json::from_str(&json)?;
        assert_eq!(deserialized.secret(), "super-secret");

        Ok(())
    }
}
