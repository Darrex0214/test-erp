use thiserror::Error;

/// Resultado estándar para operaciones de dominio.
pub type DomainResult<T> = Result<T, DomainError>;

/// Errores de negocio del ERP.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// Error de validación de una regla de negocio.
    #[error("Validación de dominio fallida: {0}")]
    Validation(String),
    /// La entidad solicitada no existe.
    #[error("Entidad no encontrada: {0}")]
    NotFound(String),
    /// La operación no es válida para el estado actual.
    #[error("Operación inválida: {0}")]
    InvalidOperation(String),
    /// Se intentó operar con monedas diferentes.
    #[error("Las monedas no coinciden: {left} != {right}")]
    CurrencyMismatch {
        /// Moneda izquierda.
        left: String,
        /// Moneda derecha.
        right: String,
    },
}
