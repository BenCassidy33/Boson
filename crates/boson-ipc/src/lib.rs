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
pub type ProcessRecv = mpsc::Receiver<IpcMessageWrapper>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Process {
    Photon,
    Collider,
    Spectra,
}

pub struct IpcHub {
    routes: RwLock<HashMap<Process, mpsc::Sender<IpcMessageWrapper>>>,
    next_id: u64,
}

impl IpcHub {
    pub fn new() -> Self {
        Self {
            routes: RwLock::new(HashMap::new()),
            next_id: 0,
        }
    }

    pub fn register(&self, process: Process) -> ProcessRecv {
        let (tx, rx) = mpsc::channel(128);
        self.routes.write().unwrap().insert(process, tx);

        rx
    }

    pub fn send<T: IpcMessage>(&self, to: Process, payload: T) -> anyhow::Result<()> {
        let sender = {
            let routes = self.routes.read().unwrap();

            routes
                .get(&to)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Process {:?} not registered", to))?
        };

        let wrapper = IpcMessageWrapper::new(0, to, to, Box::new(payload), false);
        sender.try_send(wrapper)?;

        Ok(())
    }
}

impl Default for IpcHub {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IpcMessageWrapper {
    pub payload: IpcPayload,
    pub reply_expected: bool,

    id: u64,
    sender: Process,
    reciever: Process,
}

impl IpcMessageWrapper {
    pub fn new(
        id: u64,
        sender: Process,
        reciever: Process,
        payload: IpcPayload,
        reply_expected: bool,
    ) -> Self {
        Self {
            id,
            sender,
            reciever,
            payload,
            reply_expected,
        }
    }
}
