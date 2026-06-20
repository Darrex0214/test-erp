//! Utilidades testeables del bootstrap del servidor.

use axum::http::StatusCode;
use sqlx::PgPool;
use tracing::error;

/// Verifica conectividad básica contra PostgreSQL.
pub async fn health(pool: PgPool) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(error) => {
            error!(%error, "Health check falló");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
