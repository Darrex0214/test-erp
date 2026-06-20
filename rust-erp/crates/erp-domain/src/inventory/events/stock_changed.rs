use crate::inventory::entities::Product;
use crate::inventory::value_objects::{Quantity, Sku};
use crate::shared::{EntityId, Timestamp};

/// Evento emitido cuando cambia el stock de un producto.
#[derive(Debug, Clone)]
pub struct StockChanged {
    /// Producto afectado.
    pub product_id: EntityId<Product>,
    /// SKU del producto.
    pub sku: Sku,
    /// Cantidad anterior.
    pub previous: Quantity,
    /// Nueva cantidad.
    pub current: Quantity,
    /// Momento del cambio.
    pub changed_at: Timestamp,
}
