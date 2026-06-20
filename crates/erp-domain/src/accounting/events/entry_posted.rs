use crate::accounting::entities::JournalEntry;
use crate::shared::{EntityId, Money, Timestamp};

/// Evento emitido cuando un asiento contable se publica.
#[derive(Debug, Clone)]
pub struct EntryPosted {
    /// Identificador del asiento.
    pub entry_id: EntityId<JournalEntry>,
    /// Momento de publicación.
    pub posted_at: Timestamp,
    /// Total debitado en el asiento.
    pub total_debit: Money,
}
