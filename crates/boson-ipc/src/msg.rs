use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;

use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpcMessageId(TypeId);

impl IpcMessageId {
    pub fn new(id: TypeId) -> Self {
        Self(id)
    }
}

impl From<TypeId> for IpcMessageId {
    fn from(value: TypeId) -> Self {
        Self(value)
    }
}

pub trait IpcMessage: Any + Send + Sync {
    fn id(&self) -> IpcMessageId;
    fn as_any(&self) -> &dyn Any;

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub trait IpcMessageExt {
    fn is<T: Any>(&self) -> bool;
    fn downcast_ref<T: Any>(&self) -> Option<&T>;
}

impl IpcMessageExt for dyn IpcMessage {
    fn is<T: Any>(&self) -> bool {
        self.as_any().is::<T>()
    }

    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub trait IpcEvent: IpcMessage {}
pub trait IpcCommand: IpcMessage {}

pub type IpcPayload = Box<dyn IpcMessage>;
pub type ProcessRecv = mpsc::Receiver<crate::IpcMessageWrapper>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Process {
    Photon,
    Collider,
    Spectra,
}
