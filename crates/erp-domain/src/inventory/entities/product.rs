use crate::inventory::events::StockChanged;
use crate::inventory::value_objects::{Quantity, Sku};
use crate::shared::{DomainError, DomainResult, EntityId, Timestamp};

/// Producto inventariable como raíz de agregado.
#[derive(Debug, Clone)]
pub struct Product {
    id: EntityId<Product>,
    sku: Sku,
    name: String,
    stock: Quantity,
    active: bool,
    events: Vec<StockChanged>,
}

impl Product {
    /// Crea un producto activo sin stock inicial.
    pub fn new(sku: Sku, name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "El nombre del producto no puede estar vacío".to_owned(),
            ));
        }

        Ok(Self {
            id: EntityId::new(),
            sku,
            name,
            stock: Quantity::zero(),
            active: true,
            events: Vec::new(),
        })
    }

    /// Reconstruye un producto desde persistencia.
    #[must_use]
    pub fn rehydrate(
        id: EntityId<Product>,
        sku: Sku,
        name: String,
        stock: Quantity,
        active: bool,
    ) -> Self {
        Self {
            id,
            sku,
            name,
            stock,
            active,
            events: Vec::new(),
        }
    }

    /// Cambia el nombre del producto.
    pub fn rename(&mut self, name: impl Into<String>) -> DomainResult<()> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "El nombre del producto no puede estar vacío".to_owned(),
            ));
        }
        self.name = name;
        Ok(())
    }

    /// Ajusta el stock con una variación positiva o negativa.
    pub fn adjust_stock(&mut self, delta: i64) -> DomainResult<()> {
        if !self.active {
            return Err(DomainError::InvalidOperation(
                "No se puede ajustar stock de un producto inactivo".to_owned(),
            ));
        }

        let previous = self.stock;
        let next = previous.value().checked_add(delta).ok_or_else(|| {
            DomainError::Validation("El ajuste de inventario excede el rango permitido".to_owned())
        })?;
        self.stock = Quantity::new(next)?;
        self.events.push(StockChanged {
            product_id: self.id,
            sku: self.sku.clone(),
            previous,
            current: self.stock,
            changed_at: Timestamp::now(),
        });
        Ok(())
    }

    /// Desactiva el producto.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Extrae y limpia los eventos pendientes.
    #[must_use]
    pub fn drain_events(&mut self) -> Vec<StockChanged> {
        core::mem::take(&mut self.events)
    }

    /// Identificador del producto.
    #[must_use]
    pub fn id(&self) -> EntityId<Product> {
        self.id
    }

    /// SKU del producto.
    #[must_use]
    pub fn sku(&self) -> &Sku {
        &self.sku
    }

    /// Nombre del producto.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Stock actual.
    #[must_use]
    pub const fn stock(&self) -> Quantity {
        self.stock
    }

    /// Indica si el producto está activo.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.active
    }
}
