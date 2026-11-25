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

use tokio::sync::mpsc;

use crate::{ctx::GluonProcessContext, msg::GluonCall, proc::GluonProcess};

pub struct GluonRoute {
    pub type_id: TypeId,
    pub sender: mpsc::Sender<GluonCall>,
}

pub enum GluonHubError<R: Eq + Hash> {
    DuplicateProcess(R),
}

pub struct GluonHub<R: Eq + Hash> {
    routes: Arc<Mutex<HashMap<R, GluonRoute>>>,
}

impl<R: Eq + Hash> GluonHub<R> {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register<P: GluonProcess>(
        &mut self,
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
                eprintln!("Process Failed: {error}")
            }
        });

        Ok(())
    }
}

impl<R: Eq + Hash> Default for GluonHub<R> {
    fn default() -> Self {
        Self::new()
    }
}
