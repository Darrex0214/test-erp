use erp_domain::inventory::entities::Product;
use erp_domain::inventory::value_objects::Sku;

#[test]
fn inventory_product_adjustment_changes_stock() {
    let mut product = Product::new(Sku::new("INV-001").expect("sku"), "Monitor").expect("producto");

    product.adjust_stock(3).expect("ajustar stock");

    assert_eq!(product.stock().value(), 3);
}
