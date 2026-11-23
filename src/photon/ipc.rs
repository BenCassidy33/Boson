use boson_macros::BosonIpcMessage;

#[derive(Debug, BosonIpcMessage)]
pub struct FetchResource {
    pub url: String,
}
