use std::sync::Arc;

use erp_domain::invoicing::entities::Invoice;
use erp_domain::invoicing::ports::InvoiceRepository;
use erp_domain::shared::DomainResult;

/// Caso de uso para listar facturas.
#[derive(Clone)]
pub struct ListInvoicesUseCase {
    repository: Arc<dyn InvoiceRepository>,
}

impl ListInvoicesUseCase {
    /// Construye el caso de uso.
    #[must_use]
    pub fn new(repository: Arc<dyn InvoiceRepository>) -> Self {
        Self { repository }
    }

    /// Ejecuta la query.
    pub async fn execute(&self) -> DomainResult<Vec<Invoice>> {
        self.repository.list().await
    }
}
