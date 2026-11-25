use crate::IpcMessageWrapper;

pub struct IpcReplyHandle {}

impl IpcReplyHandle {
    fn new() -> Self {
        todo!()
    }

    pub async fn await_reply(&self) -> anyhow::Result<IpcMessageWrapper> {
        todo!()
    }
}
