use axum::extract::{Form, State};
use axum::response::Html;
use erp_application::accounting::commands::{PostJournalEntryCommand, PostJournalEntryLine};
use erp_domain::accounting::entities::Account;
use erp_domain::accounting::value_objects::PostingSide;
use erp_domain::shared::{Currency, EntityId, Money};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::dto::requests::PostJournalEntryRequest;
use crate::handlers::{WebResult, bad_request, internal_error};
use crate::router::AppState;

/// Renderiza el dashboard.
pub async fn dashboard() -> WebResult<Html<String>> {
    erp_templates::render_dashboard()
        .map(Html)
        .map_err(internal_error)
}

/// Renderiza la página contable.
pub async fn journal_page() -> WebResult<Html<String>> {
    erp_templates::render_journal()
        .map(Html)
        .map_err(internal_error)
}

/// Publica un asiento contable desde formulario HTTP.
pub async fn post_journal_entry(
    State(state): State<AppState>,
    Form(request): Form<PostJournalEntryRequest>,
) -> WebResult<String> {
    let currency = Currency::try_from(request.currency.as_str()).map_err(bad_request)?;
    let amount = request.amount.parse::<Decimal>().map_err(bad_request)?;
    let debit_account_id = request
        .debit_account_id
        .parse::<Uuid>()
        .map_err(bad_request)?;
    let credit_account_id = request
        .credit_account_id
        .parse::<Uuid>()
        .map_err(bad_request)?;

    let command = PostJournalEntryCommand {
        description: request.description,
        lines: vec![
            PostJournalEntryLine {
                account_id: EntityId::<Account>::from_uuid(debit_account_id),
                side: PostingSide::Debit,
                amount: Money::new(amount, currency),
            },
            PostJournalEntryLine {
                account_id: EntityId::<Account>::from_uuid(credit_account_id),
                side: PostingSide::Credit,
                amount: Money::new(amount, currency),
            },
        ],
    };

    let entry = state
        .post_journal_entry
        .execute(command)
        .await
        .map_err(bad_request)?;
    Ok(format!(
        "Asiento {} publicado correctamente",
        entry.id().value()
    ))
}
