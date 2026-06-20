use crate::shared::{DomainError, DomainResult};

/// Código de cuenta contable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountCode(String);

impl AccountCode {
    /// Crea un código contable validado.
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let valid = !value.trim().is_empty()
            && value.len() <= 32
            && value
                .chars()
                .all(|ch| ch.is_ascii_digit() || ch == '-' || ch == '.');

        if valid {
            Ok(Self(value))
        } else {
            Err(DomainError::Validation(
                "El código contable solo acepta dígitos, punto y guion".to_owned(),
            ))
        }
    }

    /// Valor textual del código.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}
