use tokio::sync::oneshot;

use crate::proc::GluonIdentifier;

pub trait GluonMessage: GluonIdentifier + Send {}

pub struct GluonCall {
    message: Box<dyn GluonMessage>,
    reply: oneshot::Sender<Box<dyn GluonMessage>>,
}

impl GluonCall {
    pub fn message(&self) -> &dyn GluonMessage {
        self.message.as_ref()
    }

    pub fn reply(self, response: Box<dyn GluonMessage>) -> Result<(), Box<dyn GluonMessage>> {
        self.reply.send(response)
    }
}
