//! Plantillas MiniJinja del ERP.

use minijinja::{Environment, context};
use serde::Serialize;

/// Producto serializable para plantillas.
#[derive(Debug, Clone, Serialize)]
pub struct ProductView {
    /// SKU.
    pub sku: String,
    /// Nombre.
    pub name: String,
    /// Stock.
    pub stock: i64,
}

/// Factura serializable para plantillas.
#[derive(Debug, Clone, Serialize)]
pub struct InvoiceView {
    /// Número.
    pub number: String,
    /// Estado.
    pub status: String,
    /// Total.
    pub total: String,
}

/// Renderiza el dashboard.
pub fn render_dashboard() -> Result<String, minijinja::Error> {
    render("dashboard.html", context! {})
}

/// Renderiza la página de contabilidad.
pub fn render_journal() -> Result<String, minijinja::Error> {
    render("accounting/journal.html", context! {})
}

/// Renderiza la página de productos.
pub fn render_products(products: &[ProductView]) -> Result<String, minijinja::Error> {
    render("inventory/products.html", context! { products => products })
}

/// Renderiza el parcial HTMX de productos.
pub fn render_product_list(products: &[ProductView]) -> Result<String, minijinja::Error> {
    render(
        "inventory/product-list.html",
        context! { products => products },
    )
}

/// Renderiza la página de facturas.
pub fn render_invoices(invoices: &[InvoiceView]) -> Result<String, minijinja::Error> {
    render("invoicing/invoices.html", context! { invoices => invoices })
}

fn render(template_name: &str, context: impl serde::Serialize) -> Result<String, minijinja::Error> {
    let env = environment()?;
    env.get_template(template_name)?.render(context)
}

fn environment() -> Result<Environment<'static>, minijinja::Error> {
    let mut env = Environment::new();
    env.add_template("base.html", include_str!("../templates/base.html"))?;
    env.add_template(
        "dashboard.html",
        include_str!("../templates/dashboard.html"),
    )?;
    env.add_template(
        "accounting/journal.html",
        include_str!("../templates/accounting/journal.html"),
    )?;
    env.add_template(
        "inventory/products.html",
        include_str!("../templates/inventory/products.html"),
    )?;
    env.add_template(
        "inventory/product-list.html",
        include_str!("../templates/inventory/product-list.html"),
    )?;
    env.add_template(
        "invoicing/invoices.html",
        include_str!("../templates/invoicing/invoices.html"),
    )?;
    Ok(env)
}
