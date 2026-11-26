use gluon::GluonMessage;

use crate::spectra::Frame;

#[derive(Debug, GluonMessage)]
pub struct RequestFrameData {}

#[derive(Debug, GluonMessage)]
pub struct ReplyFrameData {
    pub inner: Vec<Frame>,
}
