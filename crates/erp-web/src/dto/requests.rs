use serde::Deserialize;

/// Formulario para publicar asiento contable.
#[derive(Debug, Deserialize)]
pub struct PostJournalEntryRequest {
    /// Descripción del asiento.
    pub description: String,
    /// Cuenta de débito en UUID.
    pub debit_account_id: String,
    /// Cuenta de crédito en UUID.
    pub credit_account_id: String,
    /// Monto decimal.
    pub amount: String,
    /// Código de moneda.
    pub currency: String,
}

/// Formulario para ajustar stock.
#[derive(Debug, Deserialize)]
pub struct AdjustStockRequest {
    /// Producto en UUID.
    pub product_id: String,
    /// Variación de stock.
    pub delta: i64,
}

/// Formulario para emitir factura con una línea inicial.
#[derive(Debug, Deserialize)]
pub struct IssueInvoiceRequest {
    /// Número de factura.
    pub number: String,
    /// Cliente en UUID.
    pub customer_id: String,
    /// Descripción de línea.
    pub description: String,
    /// Cantidad.
    pub quantity: u32,
    /// Precio unitario.
    pub unit_price: String,
    /// Código de moneda.
    pub currency: String,
}
