use erp_domain::invoicing::entities::{Customer, Invoice, InvoiceLine, InvoiceStatus};
use erp_domain::invoicing::value_objects::InvoiceNumber;
use erp_domain::shared::{Currency, EntityId, Money};
use rust_decimal::Decimal;

#[test]
fn invoicing_invoice_can_be_issued() {
    let mut invoice = Invoice::draft(
        InvoiceNumber::new("FAC-001").expect("número"),
        EntityId::<Customer>::new(),
    );
    invoice
        .add_line(
            InvoiceLine::new(
                "Licencia",
                1,
                Money::new(Decimal::new(9900, 2), Currency::Usd),
            )
            .expect("línea"),
        )
        .expect("agregar línea");

    invoice.issue().expect("emitir");

    assert_eq!(invoice.status(), InvoiceStatus::Issued);
}
