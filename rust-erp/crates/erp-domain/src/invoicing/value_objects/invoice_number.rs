use crate::shared::{DomainError, DomainResult};

/// Número único de factura.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvoiceNumber(String);

impl InvoiceNumber {
    /// Crea un número de factura validado.
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let valid = (3..=40).contains(&value.len())
            && value
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '/');

        if valid {
            Ok(Self(value))
        } else {
            Err(DomainError::Validation(
                "El número de factura debe tener 3 a 40 caracteres válidos".to_owned(),
            ))
        }
    }

    /// Valor textual del número.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}
