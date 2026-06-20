use crate::accounting::value_objects::{Posting, PostingSide};
use crate::shared::{DomainError, DomainResult, Money};

/// Servicio de dominio para validar partida doble.
pub struct DoubleEntryService;

impl DoubleEntryService {
    /// Verifica que la suma de débitos sea igual a la suma de créditos.
    pub fn validate_balanced(postings: &[Posting]) -> DomainResult<()> {
        let Some(first) = postings.first().copied() else {
            return Err(DomainError::Validation(
                "Un asiento debe tener al menos dos partidas".to_owned(),
            ));
        };

        let currency = first.amount().currency();
        let mut debits = Money::zero(currency);
        let mut credits = Money::zero(currency);

        for posting in postings {
            match posting.side() {
                PostingSide::Debit => debits = debits.add(posting.amount())?,
                PostingSide::Credit => credits = credits.add(posting.amount())?,
            }
        }

        if debits.amount() == credits.amount() {
            Ok(())
        } else {
            Err(DomainError::Validation(
                "El asiento no cumple partida doble: débitos y créditos difieren".to_owned(),
            ))
        }
    }
}
