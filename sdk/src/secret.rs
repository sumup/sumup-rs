/// Wrapper for password values that keeps the inner string private.
#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Secret {
    secret: String,
}

impl Secret {
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

impl From<String> for Secret {
    fn from(secret: String) -> Self {
        Self::new(secret)
    }
}

impl From<&str> for Secret {
    fn from(secret: &str) -> Self {
        Self::new(secret)
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secret").field("secret", &"***").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_secret() {
        let password = Secret::new("super-secret");

        assert_eq!(password.secret(), "super-secret");
    }

    #[test]
    fn into_secret_consumes_password() {
        let password = Secret::new("super-secret");

        let secret = password.into_secret();

        assert_eq!(secret, "super-secret");
    }

    #[test]
    fn from_str_constructs_password() {
        let password: Secret = "super-secret".into();

        assert_eq!(password.secret(), "super-secret");
    }

    #[test]
    fn debug_masks_secret() {
        let password = Secret::new("super-secret");
        let debug_output = format!("{password:?}");

        assert_eq!(debug_output, "Secret { secret: \"***\" }");
        assert!(!debug_output.contains("super-secret"));
    }

    #[test]
    fn serde_round_trip_preserves_secret() -> Result<(), serde_json::Error> {
        let password = Secret::new("super-secret");

        let json = serde_json::to_string(&password)?;
        assert_eq!(json, "\"super-secret\"");

        let deserialized: Secret = serde_json::from_str(&json)?;
        assert_eq!(deserialized.secret(), "super-secret");

        Ok(())
    }
}
