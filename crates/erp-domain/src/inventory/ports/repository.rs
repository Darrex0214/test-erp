use async_trait::async_trait;

use crate::inventory::entities::Product;
use crate::shared::{DomainResult, EntityId};

/// Puerto de persistencia para productos.
#[async_trait]
pub trait ProductRepository: Send + Sync {
    /// Guarda un producto.
    async fn save(&self, product: &Product) -> DomainResult<()>;

    /// Busca un producto por identificador.
    async fn find_by_id(&self, id: EntityId<Product>) -> DomainResult<Option<Product>>;

    /// Lista productos registrados.
    async fn list(&self) -> DomainResult<Vec<Product>>;
}
