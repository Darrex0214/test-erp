use std::sync::Arc;

use erp_domain::invoicing::entities::{Customer, Invoice, InvoiceLine};
use erp_domain::invoicing::ports::InvoiceRepository;
use erp_domain::invoicing::value_objects::InvoiceNumber;
use erp_domain::shared::{DomainResult, EntityId, Money};

/// Línea del command para emitir factura.
#[derive(Debug, Clone)]
pub struct IssueInvoiceLineCommand {
    /// Descripción.
    pub description: String,
    /// Cantidad.
    pub quantity: u32,
    /// Precio unitario.
    pub unit_price: Money,
}

/// Command para emitir factura.
#[derive(Debug, Clone)]
pub struct IssueInvoiceCommand {
    /// Número de factura.
    pub number: InvoiceNumber,
    /// Cliente.
    pub customer_id: EntityId<Customer>,
    /// Líneas.
    pub lines: Vec<IssueInvoiceLineCommand>,
}

/// Caso de uso que orquesta creación, validación y persistencia de factura.
#[derive(Clone)]
pub struct IssueInvoiceUseCase {
    repository: Arc<dyn InvoiceRepository>,
}

impl IssueInvoiceUseCase {
    /// Construye el caso de uso.
    #[must_use]
    pub fn new(repository: Arc<dyn InvoiceRepository>) -> Self {
        Self { repository }
    }

    /// Ejecuta el command.
    pub async fn execute(&self, command: IssueInvoiceCommand) -> DomainResult<Invoice> {
        let mut invoice = Invoice::draft(command.number, command.customer_id);

        for line in command.lines {
            invoice.add_line(InvoiceLine::new(
                line.description,
                line.quantity,
                line.unit_price,
            )?)?;
        }

        invoice.issue()?;
        self.repository.save(&invoice).await?;
        Ok(invoice)
    }
}
