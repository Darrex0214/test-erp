/// Monedas soportadas por el ERP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    /// Dólar estadounidense.
    Usd,
    /// Euro.
    Eur,
    /// Peso mexicano.
    Mxn,
}

impl Currency {
    /// Código ISO de la moneda.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Usd => "USD",
            Self::Eur => "EUR",
            Self::Mxn => "MXN",
        }
    }
}

impl TryFrom<&str> for Currency {
    type Error = crate::shared::DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "USD" => Ok(Self::Usd),
            "EUR" => Ok(Self::Eur),
            "MXN" => Ok(Self::Mxn),
            other => Err(crate::shared::DomainError::Validation(format!(
                "Moneda no soportada: {other}"
            ))),
        }
    }
}
