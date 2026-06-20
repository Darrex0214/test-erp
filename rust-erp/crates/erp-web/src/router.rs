use axum::Router;
use axum::routing::{get, post};
use erp_application::accounting::commands::PostJournalEntryUseCase;
use erp_application::inventory::commands::AdjustStockUseCase;
use erp_application::inventory::queries::ListProductsUseCase;
use erp_application::invoicing::commands::IssueInvoiceUseCase;
use erp_application::invoicing::queries::ListInvoicesUseCase;

use crate::handlers::{accounting, inventory, invoicing};

/// Estado HTTP compartido por handlers.
#[derive(Clone)]
pub struct AppState {
    /// Command para publicar asientos.
    pub post_journal_entry: PostJournalEntryUseCase,
    /// Command para ajustar inventario.
    pub adjust_stock: AdjustStockUseCase,
    /// Query para listar productos.
    pub list_products: ListProductsUseCase,
    /// Command para emitir facturas.
    pub issue_invoice: IssueInvoiceUseCase,
    /// Query para listar facturas.
    pub list_invoices: ListInvoicesUseCase,
}

/// Crea el router de presentación.
#[must_use]
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(accounting::dashboard))
        .route("/accounting", get(accounting::journal_page))
        .route(
            "/accounting/journal-entries",
            post(accounting::post_journal_entry),
        )
        .route("/inventory", get(inventory::products_page))
        .route("/inventory/products", get(inventory::product_list_partial))
        .route("/inventory/stock", post(inventory::adjust_stock))
        .route("/invoicing", get(invoicing::invoices_page))
        .route("/invoicing/invoices", post(invoicing::issue_invoice))
        .with_state(state)
}
