use std::sync::Arc;

use erp_domain::accounting::entities::{Account, JournalEntry};
use erp_domain::accounting::ports::JournalEntryRepository;
use erp_domain::accounting::value_objects::{Posting, PostingSide};
use erp_domain::shared::{DomainResult, EntityId, Money};

/// Línea del command para publicar un asiento.
#[derive(Debug, Clone)]
pub struct PostJournalEntryLine {
    /// Cuenta afectada.
    pub account_id: EntityId<Account>,
    /// Lado contable.
    pub side: PostingSide,
    /// Monto.
    pub amount: Money,
}

/// Command para publicar un asiento contable.
#[derive(Debug, Clone)]
pub struct PostJournalEntryCommand {
    /// Descripción del asiento.
    pub description: String,
    /// Partidas contables.
    pub lines: Vec<PostJournalEntryLine>,
}

/// Caso de uso para publicar asientos contables.
#[derive(Clone)]
pub struct PostJournalEntryUseCase {
    repository: Arc<dyn JournalEntryRepository>,
}

impl PostJournalEntryUseCase {
    /// Construye el caso de uso.
    #[must_use]
    pub fn new(repository: Arc<dyn JournalEntryRepository>) -> Self {
        Self { repository }
    }

    /// Ejecuta el command.
    pub async fn execute(&self, command: PostJournalEntryCommand) -> DomainResult<JournalEntry> {
        let mut entry = JournalEntry::draft(command.description)?;
        for line in command.lines {
            entry.add_posting(Posting::new(line.account_id, line.side, line.amount)?)?;
        }
        entry.post()?;
        self.repository.save(&entry).await?;
        Ok(entry)
    }
}
