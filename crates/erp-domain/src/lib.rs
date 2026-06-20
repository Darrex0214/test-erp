//! Núcleo de dominio del ERP.

pub mod accounting;
pub mod inventory;
pub mod invoicing;
pub mod shared;

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::accounting::entities::Account;
    use crate::accounting::services::DoubleEntryService;
    use crate::accounting::value_objects::{AccountCode, Posting};
    use crate::inventory::entities::Product;
    use crate::inventory::value_objects::Sku;
    use crate::invoicing::entities::{Customer, Invoice, InvoiceLine, InvoiceStatus};
    use crate::invoicing::value_objects::{InvoiceNumber, TaxId};
    use crate::shared::{Currency, EntityId, Money};

    #[test]
    fn money_adds_same_currency() {
        let left = Money::new(Decimal::new(1000, 2), Currency::Usd);
        let right = Money::new(Decimal::new(250, 2), Currency::Usd);

        let result = left.add(right).expect("debe sumar");

        assert_eq!(result.amount(), Decimal::new(1250, 2));
        assert_eq!(result.currency(), Currency::Usd);
    }

    #[test]
    fn money_rejects_different_currencies() {
        let left = Money::new(Decimal::new(1000, 2), Currency::Usd);
        let right = Money::new(Decimal::new(250, 2), Currency::Mxn);

        assert!(left.add(right).is_err());
    }

    #[test]
    fn money_subtracts_same_currency() {
        let left = Money::new(Decimal::new(1000, 2), Currency::Usd);
        let right = Money::new(Decimal::new(250, 2), Currency::Usd);

        let result = left.subtract(right).expect("debe restar");

        assert_eq!(result.amount(), Decimal::new(750, 2));
    }

    #[test]
    fn money_detects_negative_or_zero() {
        let zero = Money::new(Decimal::ZERO, Currency::Usd);
        let negative = Money::new(Decimal::new(-1, 0), Currency::Usd);

        assert!(zero.is_negative_or_zero());
        assert!(negative.is_negative_or_zero());
    }

    #[test]
    fn account_code_rejects_invalid_characters() {
        assert!(AccountCode::new("10-01.02").is_ok());
        assert!(AccountCode::new("ventas").is_err());
    }

    #[test]
    fn double_entry_accepts_balanced_postings() {
        let debit_account = EntityId::<Account>::new();
        let credit_account = EntityId::<Account>::new();
        let amount = Money::new(Decimal::new(10000, 2), Currency::Usd);
        let postings = vec![
            Posting::debit(debit_account, amount).expect("débito válido"),
            Posting::credit(credit_account, amount).expect("crédito válido"),
        ];

        assert!(DoubleEntryService::validate_balanced(&postings).is_ok());
    }

    #[test]
    fn double_entry_rejects_unbalanced_postings() {
        let debit_account = EntityId::<Account>::new();
        let credit_account = EntityId::<Account>::new();
        let postings = vec![
            Posting::debit(
                debit_account,
                Money::new(Decimal::new(10000, 2), Currency::Usd),
            )
            .expect("débito válido"),
            Posting::credit(
                credit_account,
                Money::new(Decimal::new(9000, 2), Currency::Usd),
            )
            .expect("crédito válido"),
        ];

        assert!(DoubleEntryService::validate_balanced(&postings).is_err());
    }

    #[test]
    fn sku_requires_business_format() {
        assert!(Sku::new("ERP-001").is_ok());
        assert!(Sku::new("x").is_err());
    }

    #[test]
    fn product_stock_cannot_become_negative() {
        let mut product =
            Product::new(Sku::new("ERP-001").expect("sku"), "Teclado").expect("producto");

        assert!(product.adjust_stock(-1).is_err());
    }

    #[test]
    fn invoice_issue_emits_event() {
        let customer_id = EntityId::<Customer>::new();
        let mut invoice = Invoice::draft(InvoiceNumber::new("F-001").expect("número"), customer_id);
        invoice
            .add_line(
                InvoiceLine::new(
                    "Servicio",
                    2,
                    Money::new(Decimal::new(1500, 2), Currency::Usd),
                )
                .expect("línea"),
            )
            .expect("agregar línea");

        invoice.issue().expect("emitir");

        assert_eq!(invoice.status(), InvoiceStatus::Issued);
        assert_eq!(invoice.drain_events().len(), 1);
    }

    #[test]
    fn invoice_cannot_cancel_paid_invoice() {
        let customer_id = EntityId::<Customer>::new();
        let mut invoice = Invoice::draft(InvoiceNumber::new("F-002").expect("número"), customer_id);
        invoice
            .add_line(
                InvoiceLine::new(
                    "Servicio",
                    1,
                    Money::new(Decimal::new(1000, 2), Currency::Usd),
                )
                .expect("línea"),
            )
            .expect("agregar línea");
        invoice.issue().expect("emitir");
        invoice.mark_paid().expect("pagar");

        assert!(invoice.cancel().is_err());
    }

    #[test]
    fn customer_requires_name_and_tax_id_format() {
        let tax_id = TaxId::new("RFC-123456").expect("tax id");

        assert!(Customer::new("Cliente SA", tax_id, None).is_ok());
    }
}
