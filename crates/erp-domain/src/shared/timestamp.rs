use chrono::{DateTime, Utc};

/// Marca temporal inmutable del dominio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Crea una marca temporal con la hora actual.
    #[must_use]
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Reconstruye una marca temporal desde `DateTime<Utc>`.
    #[must_use]
    pub const fn from_datetime(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    /// Devuelve el valor interno.
    #[must_use]
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}
