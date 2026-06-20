use async_trait::async_trait;

use crate::accounting::entities::JournalEntry;
use crate::shared::{DomainResult, EntityId};

/// Puerto de persistencia para asientos contables.
#[async_trait]
pub trait JournalEntryRepository: Send + Sync {
    /// Guarda un asiento contable.
    async fn save(&self, entry: &JournalEntry) -> DomainResult<()>;

    /// Busca un asiento por identificador.
    async fn find_by_id(&self, id: EntityId<JournalEntry>) -> DomainResult<Option<JournalEntry>>;

    /// Lista los asientos publicados.
    async fn list_posted(&self) -> DomainResult<Vec<JournalEntry>>;
}
