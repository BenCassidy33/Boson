use boson_macros::BosonIpcMessage;

use crate::spectra::Frame;

#[derive(Debug, BosonIpcMessage)]
pub struct RequestFrameData {}

#[derive(Debug, BosonIpcMessage)]
pub struct ReplyFrameData {
    pub inner: Vec<Frame>
}
