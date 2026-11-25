use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::msg::GluonCall;

pub struct GluonProcessContext {
    reciever: mpsc::Receiver<GluonCall>,
    shutdown: CancellationToken,
}

impl GluonProcessContext {
    pub fn new(reciever: mpsc::Receiver<GluonCall>) -> Self {
        Self {
            reciever,
            shutdown: CancellationToken::new(),
        }
    }

    pub async fn recv(&mut self) -> Option<GluonCall> {
        self.reciever.recv().await
    }

    pub fn shutdown_token(&self) -> CancellationToken {
        self.shutdown.clone()
    }
}
