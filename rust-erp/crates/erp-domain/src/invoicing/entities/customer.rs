use crate::invoicing::value_objects::TaxId;
use crate::shared::{DomainError, DomainResult, EntityId};

/// Cliente facturable.
#[derive(Debug, Clone)]
pub struct Customer {
    id: EntityId<Customer>,
    name: String,
    tax_id: TaxId,
    email: Option<String>,
}

impl Customer {
    /// Crea un cliente.
    pub fn new(
        name: impl Into<String>,
        tax_id: TaxId,
        email: Option<String>,
    ) -> DomainResult<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "El nombre del cliente no puede estar vacío".to_owned(),
            ));
        }

        Ok(Self {
            id: EntityId::new(),
            name,
            tax_id,
            email,
        })
    }

    /// Reconstruye un cliente desde persistencia.
    #[must_use]
    pub fn rehydrate(
        id: EntityId<Customer>,
        name: String,
        tax_id: TaxId,
        email: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            tax_id,
            email,
        }
    }

    /// Identificador del cliente.
    #[must_use]
    pub fn id(&self) -> EntityId<Customer> {
        self.id
    }

    /// Nombre del cliente.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Identificación fiscal.
    #[must_use]
    pub fn tax_id(&self) -> &TaxId {
        &self.tax_id
    }

    /// Correo electrónico opcional.
    #[must_use]
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}
