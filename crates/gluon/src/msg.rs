use crate::proc::GluonIdentifier;
use tokio::sync::oneshot;

pub trait GluonMessage: GluonIdentifier + Send {}

pub struct GluonCall {
    message: Box<dyn GluonMessage>,
    reply: Option<oneshot::Sender<Box<dyn GluonMessage>>>,
}

impl GluonCall {
    pub fn new(
        message: Box<dyn GluonMessage>,
        reply: Option<oneshot::Sender<Box<dyn GluonMessage>>>,
    ) -> Self {
        Self { message, reply }
    }

    pub fn message(&self) -> &dyn GluonMessage {
        self.message.as_ref()
    }

    pub fn reply(self, response: Box<dyn GluonMessage>) -> Result<(), Box<dyn GluonMessage>> {
        match self.reply {
            Some(tx) => tx.send(response),
            _ => Ok(()),
        }
    }

    pub fn reply_expected(&self) -> bool {
        self.reply.is_some()
    }
}
