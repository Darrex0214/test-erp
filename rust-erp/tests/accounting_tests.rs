use erp_domain::accounting::entities::Account;
use erp_domain::accounting::services::DoubleEntryService;
use erp_domain::accounting::value_objects::Posting;
use erp_domain::shared::{Currency, EntityId, Money};
use rust_decimal::Decimal;

#[test]
fn accounting_double_entry_integration_rule_is_balanced() {
    let amount = Money::new(Decimal::new(5000, 2), Currency::Usd);
    let postings = vec![
        Posting::debit(EntityId::<Account>::new(), amount).expect("débito válido"),
        Posting::credit(EntityId::<Account>::new(), amount).expect("crédito válido"),
    ];

    assert!(DoubleEntryService::validate_balanced(&postings).is_ok());
}
