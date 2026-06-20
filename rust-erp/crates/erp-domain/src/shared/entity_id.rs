use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use uuid::Uuid;

/// Identificador fuertemente tipado para entidades de dominio.
pub struct EntityId<T> {
    value: Uuid,
    _marker: PhantomData<T>,
}

impl<T> EntityId<T> {
    /// Crea un identificador nuevo con UUID v4.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
            _marker: PhantomData,
        }
    }

    /// Reconstruye un identificador desde un UUID persistido.
    #[must_use]
    pub fn from_uuid(value: Uuid) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Devuelve el UUID interno.
    #[must_use]
    pub fn value(self) -> Uuid {
        self.value
    }
}

impl<T> Default for EntityId<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for EntityId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for EntityId<T> {}

impl<T> PartialEq for EntityId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for EntityId<T> {}

impl<T> Hash for EntityId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> fmt::Debug for EntityId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EntityId").field(&self.value).finish()
    }
}
