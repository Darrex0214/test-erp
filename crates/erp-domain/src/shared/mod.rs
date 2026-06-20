//! Shared Kernel del dominio.

pub mod currency;
pub mod entity_id;
pub mod errors;
pub mod money;
pub mod timestamp;

pub use currency::Currency;
pub use entity_id::EntityId;
pub use errors::{DomainError, DomainResult};
pub use money::Money;
pub use timestamp::Timestamp;
