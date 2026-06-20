use axum::extract::{Form, State};
use axum::response::Html;
use erp_application::inventory::commands::AdjustStockCommand;
use erp_domain::inventory::entities::Product;
use erp_domain::shared::EntityId;
use erp_templates::ProductView;
use uuid::Uuid;

use crate::dto::requests::AdjustStockRequest;
use crate::handlers::{WebResult, bad_request, internal_error};
use crate::router::AppState;

/// Renderiza la página de productos.
pub async fn products_page(State(state): State<AppState>) -> WebResult<Html<String>> {
    let products = product_views(&state).await?;
    erp_templates::render_products(&products)
        .map(Html)
        .map_err(internal_error)
}

/// Renderiza el parcial HTMX de productos.
pub async fn product_list_partial(State(state): State<AppState>) -> WebResult<Html<String>> {
    let products = product_views(&state).await?;
    erp_templates::render_product_list(&products)
        .map(Html)
        .map_err(internal_error)
}

/// Ajusta stock desde formulario HTTP.
pub async fn adjust_stock(
    State(state): State<AppState>,
    Form(request): Form<AdjustStockRequest>,
) -> WebResult<String> {
    let product_id = request.product_id.parse::<Uuid>().map_err(bad_request)?;
    let product = state
        .adjust_stock
        .execute(AdjustStockCommand {
            product_id: EntityId::<Product>::from_uuid(product_id),
            delta: request.delta,
        })
        .await
        .map_err(bad_request)?;

    Ok(format!(
        "Stock de {} ajustado a {}",
        product.sku().value(),
        product.stock().value()
    ))
}

async fn product_views(state: &AppState) -> WebResult<Vec<ProductView>> {
    let products = state.list_products.execute().await.map_err(bad_request)?;
    Ok(products
        .into_iter()
        .map(|product| ProductView {
            sku: product.sku().value().to_owned(),
            name: product.name().to_owned(),
            stock: product.stock().value(),
        })
        .collect())
}
