use crate::accounting::entities::Account;
use crate::shared::{DomainError, DomainResult, EntityId, Money};

/// Lado contable de una partida.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostingSide {
    /// Débito.
    Debit,
    /// Crédito.
    Credit,
}

/// Partida contable de un asiento.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Posting {
    account_id: EntityId<Account>,
    side: PostingSide,
    amount: Money,
}

impl Posting {
    /// Crea una partida validada.
    pub fn new(
        account_id: EntityId<Account>,
        side: PostingSide,
        amount: Money,
    ) -> DomainResult<Self> {
        if amount.is_negative_or_zero() {
            return Err(DomainError::Validation(
                "La partida contable debe tener un monto positivo".to_owned(),
            ));
        }
        Ok(Self {
            account_id,
            side,
            amount,
        })
    }

    /// Crea una partida de débito.
    pub fn debit(account_id: EntityId<Account>, amount: Money) -> DomainResult<Self> {
        Self::new(account_id, PostingSide::Debit, amount)
    }

    /// Crea una partida de crédito.
    pub fn credit(account_id: EntityId<Account>, amount: Money) -> DomainResult<Self> {
        Self::new(account_id, PostingSide::Credit, amount)
    }

    /// Cuenta afectada.
    #[must_use]
    pub const fn account_id(self) -> EntityId<Account> {
        self.account_id
    }

    /// Lado contable.
    #[must_use]
    pub const fn side(self) -> PostingSide {
        self.side
    }

    /// Monto de la partida.
    #[must_use]
    pub const fn amount(self) -> Money {
        self.amount
    }
}
