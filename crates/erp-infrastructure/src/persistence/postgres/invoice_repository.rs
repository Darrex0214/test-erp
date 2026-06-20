use async_trait::async_trait;
use chrono::{DateTime, Utc};
use erp_domain::invoicing::entities::{Customer, Invoice, InvoiceLine, InvoiceStatus};
use erp_domain::invoicing::ports::InvoiceRepository;
use erp_domain::invoicing::value_objects::InvoiceNumber;
use erp_domain::shared::{Currency, DomainResult, EntityId, Money, Timestamp};
use rust_decimal::Decimal;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::persistence_error;

/// Repositorio PostgreSQL de facturas.
#[derive(Debug, Clone)]
pub struct PostgresInvoiceRepository {
    pool: PgPool,
}

#[derive(Debug, FromRow)]
struct InvoiceRow {
    id: Uuid,
    number: String,
    customer_id: Uuid,
    status: String,
    issued_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
struct InvoiceLineRow {
    description: String,
    quantity: i32,
    unit_price: Decimal,
    currency: String,
}

impl PostgresInvoiceRepository {
    /// Construye el adapter.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn load_lines(&self, invoice_id: EntityId<Invoice>) -> DomainResult<Vec<InvoiceLine>> {
        let rows = sqlx::query_as::<_, InvoiceLineRow>(
            r#"
            SELECT description, quantity, unit_price, currency
            FROM invoice_lines
            WHERE invoice_id = $1
            ORDER BY id
            "#,
        )
        .bind(invoice_id.value())
        .fetch_all(&self.pool)
        .await
        .map_err(persistence_error)?;

        rows.into_iter()
            .map(|row| {
                InvoiceLine::new(
                    row.description,
                    row.quantity as u32,
                    Money::new(row.unit_price, Currency::try_from(row.currency.as_str())?),
                )
            })
            .collect()
    }

    async fn map_row(&self, row: InvoiceRow) -> DomainResult<Invoice> {
        let id = EntityId::from_uuid(row.id);
        let lines = self.load_lines(id).await?;
        Ok(Invoice::rehydrate(
            id,
            InvoiceNumber::new(row.number)?,
            EntityId::<Customer>::from_uuid(row.customer_id),
            lines,
            InvoiceStatus::from_code(&row.status)?,
            row.issued_at.map(Timestamp::from_datetime),
        ))
    }
}

#[async_trait]
impl InvoiceRepository for PostgresInvoiceRepository {
    async fn save(&self, invoice: &Invoice) -> DomainResult<()> {
        tracing::info!(invoice_id = %invoice.id().value(), "Guardando factura");
        sqlx::query(
            r#"
            INSERT INTO invoices (id, number, customer_id, status, issued_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE
            SET number = EXCLUDED.number,
                customer_id = EXCLUDED.customer_id,
                status = EXCLUDED.status,
                issued_at = EXCLUDED.issued_at
            "#,
        )
        .bind(invoice.id().value())
        .bind(invoice.number().value())
        .bind(invoice.customer_id().value())
        .bind(invoice.status().code())
        .bind(invoice.issued_at().map(Timestamp::value))
        .execute(&self.pool)
        .await
        .map_err(persistence_error)?;

        sqlx::query("DELETE FROM invoice_lines WHERE invoice_id = $1")
            .bind(invoice.id().value())
            .execute(&self.pool)
            .await
            .map_err(persistence_error)?;

        for line in invoice.lines() {
            sqlx::query(
                r#"
                INSERT INTO invoice_lines (invoice_id, description, quantity, unit_price, currency)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(invoice.id().value())
            .bind(line.description())
            .bind(line.quantity() as i32)
            .bind(line.unit_price().amount())
            .bind(line.unit_price().currency().code())
            .execute(&self.pool)
            .await
            .map_err(persistence_error)?;
        }

        Ok(())
    }

    async fn find_by_id(&self, id: EntityId<Invoice>) -> DomainResult<Option<Invoice>> {
        let row = sqlx::query_as::<_, InvoiceRow>(
            "SELECT id, number, customer_id, status, issued_at FROM invoices WHERE id = $1",
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(persistence_error)?;

        match row {
            Some(row) => Ok(Some(self.map_row(row).await?)),
            None => Ok(None),
        }
    }

    async fn list(&self) -> DomainResult<Vec<Invoice>> {
        let rows = sqlx::query_as::<_, InvoiceRow>(
            "SELECT id, number, customer_id, status, issued_at FROM invoices ORDER BY number",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(persistence_error)?;

        let mut invoices = Vec::with_capacity(rows.len());
        for row in rows {
            invoices.push(self.map_row(row).await?);
        }
        Ok(invoices)
    }
}
