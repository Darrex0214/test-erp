use std::sync::Arc;

use erp_domain::inventory::entities::Product;
use erp_domain::inventory::ports::ProductRepository;
use erp_domain::shared::DomainResult;

/// Caso de uso para listar productos.
#[derive(Clone)]
pub struct ListProductsUseCase {
    repository: Arc<dyn ProductRepository>,
}

impl ListProductsUseCase {
    /// Construye el caso de uso.
    #[must_use]
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    /// Ejecuta la query.
    pub async fn execute(&self) -> DomainResult<Vec<Product>> {
        self.repository.list().await
    }
}
