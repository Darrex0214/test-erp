use std::sync::{Arc, Mutex};

use erp_domain::accounting::events::EntryPosted;
use erp_domain::inventory::events::StockChanged;
use erp_domain::invoicing::events::InvoiceIssued;
use erp_domain::shared::{DomainError, DomainResult};

/// Publicador de eventos en memoria para pruebas y bootstrap inicial.
#[derive(Debug, Clone, Default)]
pub struct InMemoryEventPublisher {
    published: Arc<Mutex<Vec<String>>>,
}

impl InMemoryEventPublisher {
    /// Construye un publicador vacío.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Publica un evento de asiento contable.
    pub fn publish_entry_posted(&self, event: &EntryPosted) -> DomainResult<()> {
        self.push(format!("entry-posted:{}", event.entry_id.value()))
    }

    /// Publica un evento de cambio de stock.
    pub fn publish_stock_changed(&self, event: &StockChanged) -> DomainResult<()> {
        self.push(format!("stock-changed:{}", event.product_id.value()))
    }

    /// Publica un evento de factura emitida.
    pub fn publish_invoice_issued(&self, event: &InvoiceIssued) -> DomainResult<()> {
        self.push(format!("invoice-issued:{}", event.invoice_id.value()))
    }

    /// Lista los eventos publicados.
    pub fn published(&self) -> DomainResult<Vec<String>> {
        self.published
            .lock()
            .map(|events| events.clone())
            .map_err(|_| {
                DomainError::InvalidOperation("El bus de eventos está bloqueado".to_owned())
            })
    }

    fn push(&self, value: String) -> DomainResult<()> {
        tracing::info!(event = %value, "Publicando evento de dominio en memoria");
        self.published
            .lock()
            .map(|mut events| events.push(value))
            .map_err(|_| {
                DomainError::InvalidOperation("El bus de eventos está bloqueado".to_owned())
            })
    }
}
