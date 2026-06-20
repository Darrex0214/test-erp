//! Value Objects de contabilidad.

pub mod account_code;
pub mod posting;

pub use account_code::AccountCode;
pub use posting::{Posting, PostingSide};
