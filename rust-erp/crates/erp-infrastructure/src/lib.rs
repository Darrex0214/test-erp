//! Adapters concretos de infraestructura.

pub mod events;
pub mod persistence;

use erp_domain::shared::DomainError;

fn persistence_error(error: sqlx::Error) -> DomainError {
    tracing::error!(%error, "Error de persistencia");
    DomainError::InvalidOperation(format!("Error de persistencia: {error}"))
}
