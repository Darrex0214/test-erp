use crate::shared::{DomainError, DomainResult};

/// Cantidad de inventario no negativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quantity(i64);

impl Quantity {
    /// Crea una cantidad validada.
    pub fn new(value: i64) -> DomainResult<Self> {
        if value < 0 {
            Err(DomainError::Validation(
                "La cantidad de inventario no puede ser negativa".to_owned(),
            ))
        } else {
            Ok(Self(value))
        }
    }

    /// Cantidad cero.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Devuelve el valor entero.
    #[must_use]
    pub const fn value(self) -> i64 {
        self.0
    }
}
