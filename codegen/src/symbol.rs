use std::collections::BTreeMap;

/// Tracks every type-level Rust symbol emitted or imported by one generated module.
pub(crate) struct SymbolRegistry {
    module: String,
    symbols: BTreeMap<String, String>,
}

impl SymbolRegistry {
    pub(crate) fn new(module: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            symbols: BTreeMap::new(),
        }
    }

    /// Reserves a final Rust identifier and reports both origins on collision.
    pub(crate) fn reserve(
        &mut self,
        name: impl Into<String>,
        origin: impl Into<String>,
    ) -> Result<(), String> {
        let name = name.into();
        let origin = origin.into();

        if let Some(existing_origin) = self.symbols.get(&name) {
            return Err(format!(
                "Rust symbol collision in module `{}`: `{}` is reserved by {} and {}",
                self.module, name, existing_origin, origin
            ));
        }

        self.symbols.insert(name, origin);
        Ok(())
    }

    pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
        self.symbols.keys().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collision_reports_both_origins() {
        let mut symbols = SymbolRegistry::new("readers");
        symbols
            .reserve(
                "CreateCheckoutRequest",
                "component schema `CreateCheckoutRequest`",
            )
            .expect("first origin should reserve the name");

        let error = symbols
            .reserve(
                "CreateCheckoutRequest",
                "request body for operation `create_checkout`",
            )
            .expect_err("second origin should collide");

        assert!(error.contains("module `readers`"));
        assert!(error.contains("component schema `CreateCheckoutRequest`"));
        assert!(error.contains("request body for operation `create_checkout`"));
    }
}
