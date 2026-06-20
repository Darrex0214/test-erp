use std::sync::Arc;

use erp_domain::inventory::entities::Product;
use erp_domain::inventory::ports::ProductRepository;
use erp_domain::shared::{DomainError, DomainResult, EntityId};

/// Command para ajustar stock.
#[derive(Debug, Clone, Copy)]
pub struct AdjustStockCommand {
    /// Producto afectado.
    pub product_id: EntityId<Product>,
    /// Variación de stock.
    pub delta: i64,
}

/// Caso de uso para ajustar stock de producto.
#[derive(Clone)]
pub struct AdjustStockUseCase {
    repository: Arc<dyn ProductRepository>,
}

impl AdjustStockUseCase {
    /// Construye el caso de uso.
    #[must_use]
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    /// Ejecuta el command.
    pub async fn execute(&self, command: AdjustStockCommand) -> DomainResult<Product> {
        let mut product = self
            .repository
            .find_by_id(command.product_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Producto no encontrado".to_owned()))?;

        product.adjust_stock(command.delta)?;
        self.repository.save(&product).await?;
        Ok(product)
    }
}
