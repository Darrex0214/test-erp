//! Entidades de contabilidad.

pub mod account;
pub mod journal_entry;

pub use account::{Account, AccountType};
pub use journal_entry::JournalEntry;
