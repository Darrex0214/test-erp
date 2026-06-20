use async_trait::async_trait;
use chrono::{DateTime, Utc};
use erp_domain::accounting::entities::{Account, JournalEntry};
use erp_domain::accounting::ports::JournalEntryRepository;
use erp_domain::accounting::value_objects::{Posting, PostingSide};
use erp_domain::shared::{Currency, DomainResult, EntityId, Money, Timestamp};
use rust_decimal::Decimal;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::persistence_error;

/// Repositorio PostgreSQL de asientos contables.
#[derive(Debug, Clone)]
pub struct PostgresAccountRepository {
    pool: PgPool,
}

#[derive(Debug, FromRow)]
struct JournalEntryRow {
    id: Uuid,
    description: String,
    occurred_at: DateTime<Utc>,
    posted: bool,
}

#[derive(Debug, FromRow)]
struct PostingRow {
    account_id: Uuid,
    side: String,
    amount: Decimal,
    currency: String,
}

impl PostgresAccountRepository {
    /// Construye el adapter.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn load_postings(&self, entry_id: EntityId<JournalEntry>) -> DomainResult<Vec<Posting>> {
        let rows = sqlx::query_as::<_, PostingRow>(
            r#"
            SELECT account_id, side, amount, currency
            FROM journal_postings
            WHERE journal_entry_id = $1
            ORDER BY id
            "#,
        )
        .bind(entry_id.value())
        .fetch_all(&self.pool)
        .await
        .map_err(persistence_error)?;

        rows.into_iter()
            .map(|row| {
                let side = match row.side.as_str() {
                    "debit" => PostingSide::Debit,
                    "credit" => PostingSide::Credit,
                    _ => {
                        return Err(erp_domain::shared::DomainError::Validation(
                            "Lado contable no válido en persistencia".to_owned(),
                        ));
                    }
                };
                Posting::new(
                    EntityId::<Account>::from_uuid(row.account_id),
                    side,
                    Money::new(row.amount, Currency::try_from(row.currency.as_str())?),
                )
            })
            .collect()
    }

    async fn map_row(&self, row: JournalEntryRow) -> DomainResult<JournalEntry> {
        let id = EntityId::from_uuid(row.id);
        let postings = self.load_postings(id).await?;
        Ok(JournalEntry::rehydrate(
            id,
            Timestamp::from_datetime(row.occurred_at),
            row.description,
            postings,
            row.posted,
        ))
    }
}

#[async_trait]
impl JournalEntryRepository for PostgresAccountRepository {
    async fn save(&self, entry: &JournalEntry) -> DomainResult<()> {
        tracing::info!(entry_id = %entry.id().value(), "Guardando asiento contable");
        sqlx::query(
            r#"
            INSERT INTO journal_entries (id, description, occurred_at, posted)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
            SET description = EXCLUDED.description,
                occurred_at = EXCLUDED.occurred_at,
                posted = EXCLUDED.posted
            "#,
        )
        .bind(entry.id().value())
        .bind(entry.description())
        .bind(entry.occurred_at().value())
        .bind(entry.is_posted())
        .execute(&self.pool)
        .await
        .map_err(persistence_error)?;

        sqlx::query("DELETE FROM journal_postings WHERE journal_entry_id = $1")
            .bind(entry.id().value())
            .execute(&self.pool)
            .await
            .map_err(persistence_error)?;

        for posting in entry.postings() {
            let side = match posting.side() {
                PostingSide::Debit => "debit",
                PostingSide::Credit => "credit",
            };
            sqlx::query(
                r#"
                INSERT INTO journal_postings (journal_entry_id, account_id, side, amount, currency)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(entry.id().value())
            .bind(posting.account_id().value())
            .bind(side)
            .bind(posting.amount().amount())
            .bind(posting.amount().currency().code())
            .execute(&self.pool)
            .await
            .map_err(persistence_error)?;
        }

        Ok(())
    }

    async fn find_by_id(&self, id: EntityId<JournalEntry>) -> DomainResult<Option<JournalEntry>> {
        let row = sqlx::query_as::<_, JournalEntryRow>(
            "SELECT id, description, occurred_at, posted FROM journal_entries WHERE id = $1",
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

    async fn list_posted(&self) -> DomainResult<Vec<JournalEntry>> {
        let rows = sqlx::query_as::<_, JournalEntryRow>(
            r#"
            SELECT id, description, occurred_at, posted
            FROM journal_entries
            WHERE posted = TRUE
            ORDER BY occurred_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(persistence_error)?;

        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            entries.push(self.map_row(row).await?);
        }
        Ok(entries)
    }
}
