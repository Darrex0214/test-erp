use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use axum::routing::get;
use erp_application::accounting::commands::PostJournalEntryUseCase;
use erp_application::inventory::commands::AdjustStockUseCase;
use erp_application::inventory::queries::ListProductsUseCase;
use erp_application::invoicing::commands::IssueInvoiceUseCase;
use erp_application::invoicing::queries::ListInvoicesUseCase;
use erp_infrastructure::persistence::postgres::{
    PostgresAccountRepository, PostgresInvoiceRepository, PostgresProductRepository,
};
use erp_server::health;
use erp_web::{AppState, create_router};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL debe estar configurado")?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .context("No se pudo conectar a PostgreSQL")?;

    let journal_repository = Arc::new(PostgresAccountRepository::new(pool.clone()));
    let product_repository = Arc::new(PostgresProductRepository::new(pool.clone()));
    let invoice_repository = Arc::new(PostgresInvoiceRepository::new(pool.clone()));

    let state = AppState {
        post_journal_entry: PostJournalEntryUseCase::new(journal_repository),
        adjust_stock: AdjustStockUseCase::new(product_repository.clone()),
        list_products: ListProductsUseCase::new(product_repository),
        issue_invoice: IssueInvoiceUseCase::new(invoice_repository.clone()),
        list_invoices: ListInvoicesUseCase::new(invoice_repository),
    };

    let health_pool = pool.clone();
    let app = create_router(state).route("/health", get(move || health(health_pool.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("No se pudo abrir el puerto 3000")?;

    info!(%addr, "Servidor ERP iniciado");
    axum::serve(listener, app)
        .await
        .context("El servidor Axum terminó con error")
}
