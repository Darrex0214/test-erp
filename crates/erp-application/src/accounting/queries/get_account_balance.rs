use std::sync::Arc;

use erp_domain::accounting::entities::Account;
use erp_domain::accounting::ports::JournalEntryRepository;
use erp_domain::accounting::value_objects::PostingSide;
use erp_domain::shared::{Currency, DomainResult, EntityId, Money};

/// Query para calcular saldo de una cuenta.
#[derive(Debug, Clone, Copy)]
pub struct GetAccountBalanceQuery {
    /// Cuenta consultada.
    pub account_id: EntityId<Account>,
    /// Moneda esperada del saldo.
    pub currency: Currency,
}

/// Caso de uso para obtener el saldo de una cuenta.
#[derive(Clone)]
pub struct GetAccountBalanceUseCase {
    repository: Arc<dyn JournalEntryRepository>,
}

impl GetAccountBalanceUseCase {
    /// Construye el caso de uso.
    #[must_use]
    pub fn new(repository: Arc<dyn JournalEntryRepository>) -> Self {
        Self { repository }
    }

    /// Ejecuta la query.
    pub async fn execute(&self, query: GetAccountBalanceQuery) -> DomainResult<Money> {
        let entries = self.repository.list_posted().await?;
        let mut balance = Money::zero(query.currency);

        for posting in entries
            .iter()
            .flat_map(|entry| entry.postings())
            .filter(|posting| posting.account_id() == query.account_id)
        {
            balance = match posting.side() {
                PostingSide::Debit => balance.add(posting.amount())?,
                PostingSide::Credit => balance.subtract(posting.amount())?,
            };
        }

        Ok(balance)
    }
}
