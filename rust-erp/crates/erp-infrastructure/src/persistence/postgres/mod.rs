//! Adapters PostgreSQL con SQLx.

pub mod account_repository;
pub mod invoice_repository;
pub mod product_repository;

pub use account_repository::PostgresAccountRepository;
pub use invoice_repository::PostgresInvoiceRepository;
pub use product_repository::PostgresProductRepository;
