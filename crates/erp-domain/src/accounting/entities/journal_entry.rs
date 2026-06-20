use crate::accounting::events::EntryPosted;
use crate::accounting::services::DoubleEntryService;
use crate::accounting::value_objects::{Posting, PostingSide};
use crate::shared::{DomainError, DomainResult, EntityId, Timestamp};

/// Asiento contable como raíz de agregado.
#[derive(Debug, Clone)]
pub struct JournalEntry {
    id: EntityId<JournalEntry>,
    occurred_at: Timestamp,
    description: String,
    postings: Vec<Posting>,
    posted: bool,
    events: Vec<EntryPosted>,
}

impl JournalEntry {
    /// Crea un asiento en borrador.
    pub fn draft(description: impl Into<String>) -> DomainResult<Self> {
        let description = description.into();
        if description.trim().is_empty() {
            return Err(DomainError::Validation(
                "La descripción del asiento no puede estar vacía".to_owned(),
            ));
        }

        Ok(Self {
            id: EntityId::new(),
            occurred_at: Timestamp::now(),
            description,
            postings: Vec::new(),
            posted: false,
            events: Vec::new(),
        })
    }

    /// Reconstruye un asiento desde persistencia.
    #[must_use]
    pub fn rehydrate(
        id: EntityId<JournalEntry>,
        occurred_at: Timestamp,
        description: String,
        postings: Vec<Posting>,
        posted: bool,
    ) -> Self {
        Self {
            id,
            occurred_at,
            description,
            postings,
            posted,
            events: Vec::new(),
        }
    }

    /// Agrega una partida al asiento si aún no fue publicado.
    pub fn add_posting(&mut self, posting: Posting) -> DomainResult<()> {
        if self.posted {
            return Err(DomainError::InvalidOperation(
                "No se puede modificar un asiento publicado".to_owned(),
            ));
        }
        self.postings.push(posting);
        Ok(())
    }

    /// Publica el asiento y emite su evento de dominio.
    pub fn post(&mut self) -> DomainResult<()> {
        if self.posted {
            return Err(DomainError::InvalidOperation(
                "El asiento ya está publicado".to_owned(),
            ));
        }
        if self.postings.len() < 2 {
            return Err(DomainError::Validation(
                "Un asiento publicado requiere al menos dos partidas".to_owned(),
            ));
        }

        DoubleEntryService::validate_balanced(&self.postings)?;
        self.posted = true;
        let currency = self.postings[0].amount().currency();
        let total_debit = self
            .postings
            .iter()
            .filter(|posting| posting.side() == PostingSide::Debit)
            .try_fold(crate::shared::Money::zero(currency), |total, posting| {
                total.add(posting.amount())
            })?;
        self.events.push(EntryPosted {
            entry_id: self.id,
            posted_at: Timestamp::now(),
            total_debit,
        });
        Ok(())
    }

    /// Extrae y limpia los eventos pendientes.
    #[must_use]
    pub fn drain_events(&mut self) -> Vec<EntryPosted> {
        core::mem::take(&mut self.events)
    }

    /// Identificador del asiento.
    #[must_use]
    pub fn id(&self) -> EntityId<JournalEntry> {
        self.id
    }

    /// Fecha de ocurrencia.
    #[must_use]
    pub fn occurred_at(&self) -> &Timestamp {
        &self.occurred_at
    }

    /// Descripción del asiento.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Partidas del asiento.
    #[must_use]
    pub fn postings(&self) -> &[Posting] {
        &self.postings
    }

    /// Indica si el asiento fue publicado.
    #[must_use]
    pub const fn is_posted(&self) -> bool {
        self.posted
    }
}
