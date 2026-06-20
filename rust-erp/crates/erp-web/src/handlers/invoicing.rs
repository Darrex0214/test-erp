use axum::extract::{Form, State};
use axum::response::Html;
use erp_application::invoicing::commands::{IssueInvoiceCommand, IssueInvoiceLineCommand};
use erp_domain::invoicing::entities::Customer;
use erp_domain::invoicing::value_objects::InvoiceNumber;
use erp_domain::shared::{Currency, EntityId, Money};
use erp_templates::InvoiceView;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::dto::requests::IssueInvoiceRequest;
use crate::handlers::{WebResult, bad_request, internal_error};
use crate::router::AppState;

/// Renderiza la página de facturas.
pub async fn invoices_page(State(state): State<AppState>) -> WebResult<Html<String>> {
    let invoices = state.list_invoices.execute().await.map_err(bad_request)?;
    let views = invoices
        .into_iter()
        .map(|invoice| {
            let total = invoice
                .total()
                .map(|money| format!("{} {}", money.amount(), money.currency().code()))
                .unwrap_or_else(|error| error.to_string());
            InvoiceView {
                number: invoice.number().value().to_owned(),
                status: invoice.status().code().to_owned(),
                total,
            }
        })
        .collect::<Vec<_>>();

    erp_templates::render_invoices(&views)
        .map(Html)
        .map_err(internal_error)
}

/// Emite una factura desde formulario HTTP.
pub async fn issue_invoice(
    State(state): State<AppState>,
    Form(request): Form<IssueInvoiceRequest>,
) -> WebResult<String> {
    let customer_id = request.customer_id.parse::<Uuid>().map_err(bad_request)?;
    let currency = Currency::try_from(request.currency.as_str()).map_err(bad_request)?;
    let unit_price = request.unit_price.parse::<Decimal>().map_err(bad_request)?;

    let invoice = state
        .issue_invoice
        .execute(IssueInvoiceCommand {
            number: InvoiceNumber::new(request.number).map_err(bad_request)?,
            customer_id: EntityId::<Customer>::from_uuid(customer_id),
            lines: vec![IssueInvoiceLineCommand {
                description: request.description,
                quantity: request.quantity,
                unit_price: Money::new(unit_price, currency),
            }],
        })
        .await
        .map_err(bad_request)?;

    Ok(format!(
        "Factura {} emitida correctamente",
        invoice.number().value()
    ))
}
