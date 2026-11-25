use std::any::{Any, TypeId};

use crate::ctx::GluonProcessContext;

pub trait GluonIdentifier {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> GluonIdentifier for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait GluonIdentifierExt: GluonIdentifier {
    fn concrete_type_id(&self) -> TypeId {
        self.as_any().type_id()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn is<T: Any>(&self) -> bool {
        self.as_any().is::<T>()
    }

    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl<T: GluonIdentifier + ?Sized> GluonIdentifierExt for T {}

pub type GluonError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[async_trait::async_trait]
pub trait GluonProcess: GluonIdentifier + Send + 'static {
    async fn run(&mut self, ctx: GluonProcessContext) -> Result<(), GluonError>;
}
