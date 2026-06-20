use rust_decimal::Decimal;

use crate::shared::{Currency, DomainError, DomainResult};

/// Value Object monetario basado en `Decimal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    amount: Decimal,
    currency: Currency,
}

impl Money {
    /// Crea un monto monetario.
    #[must_use]
    pub const fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Crea un monto cero para una moneda.
    #[must_use]
    pub const fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    /// Suma dos montos de la misma moneda.
    pub fn add(self, other: Self) -> DomainResult<Self> {
        self.ensure_same_currency(other)?;
        Ok(Self::new(self.amount + other.amount, self.currency))
    }

    /// Resta dos montos de la misma moneda.
    pub fn subtract(self, other: Self) -> DomainResult<Self> {
        self.ensure_same_currency(other)?;
        Ok(Self::new(self.amount - other.amount, self.currency))
    }

    /// Indica si el monto es menor o igual a cero.
    #[must_use]
    pub fn is_negative_or_zero(self) -> bool {
        self.amount <= Decimal::ZERO
    }

    /// Devuelve el importe.
    #[must_use]
    pub const fn amount(self) -> Decimal {
        self.amount
    }

    /// Devuelve la moneda.
    #[must_use]
    pub const fn currency(self) -> Currency {
        self.currency
    }

    fn ensure_same_currency(self, other: Self) -> DomainResult<()> {
        if self.currency == other.currency {
            Ok(())
        } else {
            Err(DomainError::CurrencyMismatch {
                left: self.currency.code().to_owned(),
                right: other.currency.code().to_owned(),
            })
        }
    }
}
