use async_trait::async_trait;
use erp_domain::inventory::entities::Product;
use erp_domain::inventory::ports::ProductRepository;
use erp_domain::inventory::value_objects::{Quantity, Sku};
use erp_domain::shared::{DomainResult, EntityId};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::persistence_error;

/// Repositorio PostgreSQL de productos.
#[derive(Debug, Clone)]
pub struct PostgresProductRepository {
    pool: PgPool,
}

#[derive(Debug, FromRow)]
struct ProductRow {
    id: Uuid,
    sku: String,
    name: String,
    stock: i64,
    active: bool,
}

impl PostgresProductRepository {
    /// Construye el adapter.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: ProductRow) -> DomainResult<Product> {
        Ok(Product::rehydrate(
            EntityId::from_uuid(row.id),
            Sku::new(row.sku)?,
            row.name,
            Quantity::new(row.stock)?,
            row.active,
        ))
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn save(&self, product: &Product) -> DomainResult<()> {
        tracing::info!(product_id = %product.id().value(), "Guardando producto");
        sqlx::query(
            r#"
            INSERT INTO products (id, sku, name, stock, active)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE
            SET sku = EXCLUDED.sku,
                name = EXCLUDED.name,
                stock = EXCLUDED.stock,
                active = EXCLUDED.active
            "#,
        )
        .bind(product.id().value())
        .bind(product.sku().value())
        .bind(product.name())
        .bind(product.stock().value())
        .bind(product.is_active())
        .execute(&self.pool)
        .await
        .map_err(persistence_error)?;

        Ok(())
    }

    async fn find_by_id(&self, id: EntityId<Product>) -> DomainResult<Option<Product>> {
        let row = sqlx::query_as::<_, ProductRow>(
            "SELECT id, sku, name, stock, active FROM products WHERE id = $1",
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(persistence_error)?;

        row.map(Self::map_row).transpose()
    }

    async fn list(&self) -> DomainResult<Vec<Product>> {
        let rows = sqlx::query_as::<_, ProductRow>(
            "SELECT id, sku, name, stock, active FROM products ORDER BY sku",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(persistence_error)?;

        rows.into_iter().map(Self::map_row).collect()
    }
}
