//! Capa de presentación HTTP del ERP.

pub mod dto;
pub mod handlers;
pub mod router;

pub use router::{AppState, create_router};
