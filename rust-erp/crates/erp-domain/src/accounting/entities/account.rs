use crate::accounting::value_objects::AccountCode;
use crate::shared::{DomainError, DomainResult, EntityId};

/// Tipo contable de una cuenta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    /// Activo.
    Asset,
    /// Pasivo.
    Liability,
    /// Patrimonio.
    Equity,
    /// Ingreso.
    Income,
    /// Gasto.
    Expense,
}

/// Cuenta contable dentro del libro mayor.
#[derive(Debug, Clone)]
pub struct Account {
    id: EntityId<Account>,
    code: AccountCode,
    name: String,
    account_type: AccountType,
    active: bool,
}

impl Account {
    /// Crea una cuenta contable activa.
    pub fn new(
        code: AccountCode,
        name: impl Into<String>,
        account_type: AccountType,
    ) -> DomainResult<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "El nombre de la cuenta no puede estar vacío".to_owned(),
            ));
        }

        Ok(Self {
            id: EntityId::new(),
            code,
            name,
            account_type,
            active: true,
        })
    }

    /// Reconstruye una cuenta desde persistencia.
    #[must_use]
    pub fn rehydrate(
        id: EntityId<Account>,
        code: AccountCode,
        name: String,
        account_type: AccountType,
        active: bool,
    ) -> Self {
        Self {
            id,
            code,
            name,
            account_type,
            active,
        }
    }

    /// Cambia el nombre de la cuenta.
    pub fn rename(&mut self, name: impl Into<String>) -> DomainResult<()> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "El nombre de la cuenta no puede estar vacío".to_owned(),
            ));
        }
        self.name = name;
        Ok(())
    }

    /// Desactiva la cuenta.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Identificador de la cuenta.
    #[must_use]
    pub fn id(&self) -> EntityId<Account> {
        self.id
    }

    /// Código contable.
    #[must_use]
    pub fn code(&self) -> &AccountCode {
        &self.code
    }

    /// Nombre de la cuenta.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Tipo contable.
    #[must_use]
    pub const fn account_type(&self) -> AccountType {
        self.account_type
    }

    /// Indica si la cuenta está activa.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.active
    }
}
