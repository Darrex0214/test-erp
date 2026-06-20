use crate::invoicing::entities::Invoice;
use crate::invoicing::value_objects::InvoiceNumber;
use crate::shared::{EntityId, Money, Timestamp};

/// Evento emitido cuando una factura se emite.
#[derive(Debug, Clone)]
pub struct InvoiceIssued {
    /// Identificador de factura.
    pub invoice_id: EntityId<Invoice>,
    /// Número de factura.
    pub number: InvoiceNumber,
    /// Total facturado.
    pub total: Money,
    /// Momento de emisión.
    pub issued_at: Timestamp,
}
