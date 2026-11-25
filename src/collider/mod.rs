use std::sync::Arc;

use eframe::egui::util::id_type_map::TypeId;
use log::info;
use tokio::runtime::{self, Runtime};

use boson_ipc::{IpcHub, IpcPayload, ProcessRecv};

pub struct Collider {
    ipc_hub: Arc<IpcHub>,
    recv: ProcessRecv,
    client: reqwest::Client,
}

impl Collider {
    pub fn new(ipc_hub: Arc<IpcHub>, recv: ProcessRecv) -> anyhow::Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self {
            ipc_hub,
            client,
            recv,
        })
    }

    pub fn start(mut self) -> anyhow::Result<std::thread::JoinHandle<()>> {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?,
        );

        let handle = std::thread::Builder::new()
            .name("Collider Backend".into())
            .spawn(move || {
                let result = runtime.block_on(self.run());

                if let Err(err) = result {
                    log::error!("Collider exited: {err}");
                };
            })?;

        Ok(handle)
    }

    async fn run(mut self) -> anyhow::Result<()> {
        tokio::select! {
            msg = self.recv.recv() => {
                match msg {
                    Some(msg) => self.handle_message(msg).await?,

                    _ => {}
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, message: boson_ipc::IpcMessageWrapper) -> anyhow::Result<()> {
        dbg!(message.payload.type_name());
        todo!();
    }
}
