pub mod ctx;
pub mod msg;
pub mod proc;

pub use gluon_macros::*;

use std::{
    any::TypeId,
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
};

use tokio::sync::{mpsc, oneshot};

pub use crate::{
    ctx::GluonProcessContext,
    msg::{GluonCall, GluonMessage},
    proc::GluonProcess,
};

pub struct GluonRoute {
    pub type_id: TypeId,
    pub sender: mpsc::Sender<GluonCall>,
}

#[derive(Debug)]
pub enum GluonHubError<R> {
    DuplicateProcess(R),
    ProcessNotFound(R),
    ProcessUnavailable(R),
    ReplyDropped,
}

pub struct GluonHub<R>
where
    R: Eq + Hash + Clone,
{
    routes: Arc<Mutex<HashMap<R, GluonRoute>>>,
}

impl<R> GluonHub<R>
where
    R: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register<P: GluonProcess>(
        &self,
        id: impl Into<R>,
        mut process: P,
    ) -> Result<(), GluonHubError<R>> {
        let id = id.into();
        let (sender, reciever) = mpsc::channel(128);
        let ctx = GluonProcessContext::new(reciever);
        let type_id = TypeId::of::<P>();

        {
            let mut routes = self.routes.lock().unwrap();

            if routes.contains_key(&id) {
                return Err(GluonHubError::DuplicateProcess(id));
            }

            routes.insert(id, GluonRoute { type_id, sender });
        }

        tokio::spawn(async move {
            if let Err(error) = process.run(ctx).await {
                eprintln!("Process Failed! {error:?}")
            }
        });

        Ok(())
    }

    pub fn send(&self, to: &R, msg: Box<dyn GluonMessage>) -> Result<(), GluonHubError<R>> {
        let sender = {
            let routes = self.routes.lock().unwrap();

            routes
                .get(to)
                .ok_or_else(|| GluonHubError::ProcessNotFound(to.clone()))?
                .sender
                .clone()
        };

        let call = GluonCall::new(msg, None);

        sender
            .try_send(call)
            .map_err(|_| GluonHubError::ProcessUnavailable(to.clone()))?;

        Ok(())
    }

    pub async fn send_recv(
        &self,
        to: &R,
        msg: Box<dyn GluonMessage>,
    ) -> Result<Box<dyn GluonMessage>, GluonHubError<R>> {
        let sender = {
            let routes = self.routes.lock().unwrap();

            routes
                .get(to)
                .ok_or_else(|| GluonHubError::ProcessNotFound(to.clone()))?
                .sender
                .clone()
        };

        let (reply_tx, reply_rx) = oneshot::channel();
        let call = GluonCall::new(msg, Some(reply_tx));

        sender
            .send(call)
            .await
            .map_err(|_| GluonHubError::ProcessUnavailable(to.clone()))?;

        reply_rx.await.map_err(|_| GluonHubError::ReplyDropped)
    }
}

impl<R> Default for GluonHub<R>
where
    R: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
