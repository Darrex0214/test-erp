//! Handlers HTTP por contexto.

pub mod accounting;
pub mod inventory;
pub mod invoicing;

use axum::http::StatusCode;

pub(crate) type WebResult<T> = Result<T, (StatusCode, String)>;

pub(crate) fn bad_request(error: impl core::fmt::Display) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, error.to_string())
}

pub(crate) fn internal_error(error: impl core::fmt::Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
