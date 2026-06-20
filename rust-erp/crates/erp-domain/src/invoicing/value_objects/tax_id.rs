use crate::shared::{DomainError, DomainResult};

/// Identificación fiscal de un cliente.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaxId(String);

impl TaxId {
    /// Crea una identificación fiscal validada.
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let valid = (6..=32).contains(&value.len())
            && value
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '.');

        if valid {
            Ok(Self(value))
        } else {
            Err(DomainError::Validation(
                "La identificación fiscal debe tener 6 a 32 caracteres válidos".to_owned(),
            ))
        }
    }

    /// Valor textual de la identificación fiscal.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}
