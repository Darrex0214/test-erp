use crate::shared::{DomainError, DomainResult};

/// SKU único de un producto.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sku(String);

impl Sku {
    /// Crea un SKU validado.
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let valid = (3..=48).contains(&value.len())
            && value
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.');

        if valid {
            Ok(Self(value))
        } else {
            Err(DomainError::Validation(
                "El SKU debe tener 3 a 48 caracteres alfanuméricos, guion, punto o guion bajo"
                    .to_owned(),
            ))
        }
    }

    /// Valor textual del SKU.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}
