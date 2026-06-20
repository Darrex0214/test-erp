use crate::invoicing::events::InvoiceIssued;
use crate::invoicing::value_objects::InvoiceNumber;
use crate::shared::{Currency, DomainError, DomainResult, EntityId, Money, Timestamp};

use super::Customer;

/// Estado de una factura.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvoiceStatus {
    /// Borrador editable.
    Draft,
    /// Emitida y pendiente de pago.
    Issued,
    /// Pagada.
    Paid,
    /// Cancelada.
    Canceled,
}

impl InvoiceStatus {
    /// Código persistible del estado.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Issued => "issued",
            Self::Paid => "paid",
            Self::Canceled => "canceled",
        }
    }

    /// Reconstruye un estado desde persistencia.
    pub fn from_code(value: &str) -> DomainResult<Self> {
        match value {
            "draft" => Ok(Self::Draft),
            "issued" => Ok(Self::Issued),
            "paid" => Ok(Self::Paid),
            "canceled" => Ok(Self::Canceled),
            other => Err(DomainError::Validation(format!(
                "Estado de factura no válido: {other}"
            ))),
        }
    }
}

/// Línea de factura.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvoiceLine {
    description: String,
    quantity: u32,
    unit_price: Money,
}

impl InvoiceLine {
    /// Crea una línea de factura validada.
    pub fn new(
        description: impl Into<String>,
        quantity: u32,
        unit_price: Money,
    ) -> DomainResult<Self> {
        let description = description.into();
        if description.trim().is_empty() {
            return Err(DomainError::Validation(
                "La descripción de la línea no puede estar vacía".to_owned(),
            ));
        }
        if quantity == 0 {
            return Err(DomainError::Validation(
                "La cantidad facturada debe ser mayor que cero".to_owned(),
            ));
        }
        if unit_price.is_negative_or_zero() {
            return Err(DomainError::Validation(
                "El precio unitario debe ser mayor que cero".to_owned(),
            ));
        }

        Ok(Self {
            description,
            quantity,
            unit_price,
        })
    }

    /// Calcula el total de la línea.
    #[must_use]
    pub fn total(&self) -> Money {
        Money::new(
            self.unit_price.amount() * rust_decimal::Decimal::from(self.quantity),
            self.unit_price.currency(),
        )
    }

    /// Descripción de la línea.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Cantidad facturada.
    #[must_use]
    pub const fn quantity(&self) -> u32 {
        self.quantity
    }

    /// Precio unitario.
    #[must_use]
    pub const fn unit_price(&self) -> Money {
        self.unit_price
    }
}

/// Factura como raíz de agregado.
#[derive(Debug, Clone)]
pub struct Invoice {
    id: EntityId<Invoice>,
    number: InvoiceNumber,
    customer_id: EntityId<Customer>,
    lines: Vec<InvoiceLine>,
    status: InvoiceStatus,
    issued_at: Option<Timestamp>,
    events: Vec<InvoiceIssued>,
}

impl Invoice {
    /// Crea una factura en borrador.
    #[must_use]
    pub fn draft(number: InvoiceNumber, customer_id: EntityId<Customer>) -> Self {
        Self {
            id: EntityId::new(),
            number,
            customer_id,
            lines: Vec::new(),
            status: InvoiceStatus::Draft,
            issued_at: None,
            events: Vec::new(),
        }
    }

    /// Reconstruye una factura desde persistencia.
    #[must_use]
    pub fn rehydrate(
        id: EntityId<Invoice>,
        number: InvoiceNumber,
        customer_id: EntityId<Customer>,
        lines: Vec<InvoiceLine>,
        status: InvoiceStatus,
        issued_at: Option<Timestamp>,
    ) -> Self {
        Self {
            id,
            number,
            customer_id,
            lines,
            status,
            issued_at,
            events: Vec::new(),
        }
    }

    /// Agrega una línea a una factura en borrador.
    pub fn add_line(&mut self, line: InvoiceLine) -> DomainResult<()> {
        if self.status != InvoiceStatus::Draft {
            return Err(DomainError::InvalidOperation(
                "Solo una factura en borrador puede modificarse".to_owned(),
            ));
        }

        if let Some(first) = self.lines.first() {
            first
                .unit_price()
                .add(line.unit_price())
                .map(|_| ())
                .map_err(|_| {
                    DomainError::Validation(
                        "Todas las líneas de una factura deben usar la misma moneda".to_owned(),
                    )
                })?;
        }

        self.lines.push(line);
        Ok(())
    }

    /// Emite la factura.
    pub fn issue(&mut self) -> DomainResult<()> {
        if self.status != InvoiceStatus::Draft {
            return Err(DomainError::InvalidOperation(
                "Solo una factura en borrador puede emitirse".to_owned(),
            ));
        }
        if self.lines.is_empty() {
            return Err(DomainError::Validation(
                "No se puede emitir una factura sin líneas".to_owned(),
            ));
        }

        let issued_at = Timestamp::now();
        let total = self.total()?;
        self.status = InvoiceStatus::Issued;
        self.issued_at = Some(issued_at.clone());
        self.events.push(InvoiceIssued {
            invoice_id: self.id,
            number: self.number.clone(),
            total,
            issued_at,
        });
        Ok(())
    }

    /// Marca la factura como pagada.
    pub fn mark_paid(&mut self) -> DomainResult<()> {
        if self.status != InvoiceStatus::Issued {
            return Err(DomainError::InvalidOperation(
                "Solo una factura emitida puede marcarse como pagada".to_owned(),
            ));
        }
        self.status = InvoiceStatus::Paid;
        Ok(())
    }

    /// Cancela la factura si su estado lo permite.
    pub fn cancel(&mut self) -> DomainResult<()> {
        if self.status == InvoiceStatus::Paid {
            return Err(DomainError::InvalidOperation(
                "No se puede cancelar una factura pagada".to_owned(),
            ));
        }
        if self.status == InvoiceStatus::Canceled {
            return Err(DomainError::InvalidOperation(
                "La factura ya está cancelada".to_owned(),
            ));
        }
        self.status = InvoiceStatus::Canceled;
        Ok(())
    }

    /// Calcula el total de factura.
    pub fn total(&self) -> DomainResult<Money> {
        let Some(first) = self.lines.first() else {
            return Ok(Money::zero(Currency::Usd));
        };

        self.lines
            .iter()
            .try_fold(Money::zero(first.unit_price().currency()), |acc, line| {
                acc.add(line.total())
            })
    }

    /// Extrae y limpia eventos pendientes.
    #[must_use]
    pub fn drain_events(&mut self) -> Vec<InvoiceIssued> {
        core::mem::take(&mut self.events)
    }

    /// Identificador de factura.
    #[must_use]
    pub fn id(&self) -> EntityId<Invoice> {
        self.id
    }

    /// Número de factura.
    #[must_use]
    pub fn number(&self) -> &InvoiceNumber {
        &self.number
    }

    /// Cliente asociado.
    #[must_use]
    pub fn customer_id(&self) -> EntityId<Customer> {
        self.customer_id
    }

    /// Líneas de factura.
    #[must_use]
    pub fn lines(&self) -> &[InvoiceLine] {
        &self.lines
    }

    /// Estado actual.
    #[must_use]
    pub const fn status(&self) -> InvoiceStatus {
        self.status
    }

    /// Fecha de emisión.
    #[must_use]
    pub fn issued_at(&self) -> Option<&Timestamp> {
        self.issued_at.as_ref()
    }
}
