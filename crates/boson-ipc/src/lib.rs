use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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

pub trait IpcEvent: IpcMessage {}
pub trait IpcCommand: IpcMessage {}

pub type Message = Box<dyn IpcMessage>;
pub type ProcessRecv = mpsc::Receiver<Message>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Process {
    Photon,
    Electron,
}

pub struct IpcHub {
    routes: RwLock<HashMap<Process, mpsc::Sender<Message>>>,
}

impl IpcHub {
    pub fn new() -> Self {
        Self {
            routes: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, process: Process) -> ProcessRecv {
        let (tx, rx) = mpsc::channel(128);
        self.routes.write().unwrap().insert(process, tx);

        rx
    }

    pub fn send<T: IpcMessage>(&self, process: Process, message: T) -> anyhow::Result<()> {
        let sender = {
            let routes = self.routes.read().unwrap();

            routes
                .get(&process)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Process {:?} not registered", process))?
        };

        sender.try_send(Box::new(message))?;

        Ok(())
    }
}
