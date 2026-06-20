use async_trait::async_trait;

use crate::invoicing::entities::Invoice;
use crate::shared::{DomainResult, EntityId};

/// Puerto de persistencia para facturas.
#[async_trait]
pub trait InvoiceRepository: Send + Sync {
    /// Guarda una factura.
    async fn save(&self, invoice: &Invoice) -> DomainResult<()>;

    /// Busca una factura por identificador.
    async fn find_by_id(&self, id: EntityId<Invoice>) -> DomainResult<Option<Invoice>>;

    /// Lista facturas registradas.
    async fn list(&self) -> DomainResult<Vec<Invoice>>;
}
