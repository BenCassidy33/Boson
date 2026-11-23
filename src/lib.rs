#![allow(warnings)]

use std::{any, sync::Arc};

use boson_macros::BosonIpcMessage;
use eframe::egui::mutex::Mutex;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use scraper::Html;

use crate::{electron::Electron, photon::Photon};
use boson_ipc::{IpcHub, IpcMessage, ProcessRecv};

pub mod electron;
pub mod photon;

pub const DEFAULT_HTML: &str = include_str!("../resources/test_html.html");
pub const DEFAULT_STYLESHEET: &str = include_str!("../resources/default_styles.css");

#[inline]
pub fn default_html() -> Html {
    Html::parse_document(DEFAULT_HTML)
}

#[inline]
pub fn default_stylesheet<'a>(opts: Option<ParserOptions<'a>>) -> StyleSheet<'a> {
    StyleSheet::parse(DEFAULT_STYLESHEET, opts.unwrap_or_default())
        .expect("Failed to parse default stylesheet")
}

#[macro_export]
macro_rules! time_function {
    ($msg:expr, $body:expr) => {{
        let start = std::time::Instant::now();
        let result = $body;
        log::info!("{} {:?}", $msg, start.elapsed());

        result
    }};
}

pub struct Boson {
    hub: Arc<IpcHub>,
    photon: Photon<'static>,
    electron: Electron,
}

#[derive(Debug, BosonIpcMessage)]
pub enum GlobalIPCMessage {
    Shutdown,
}

impl Boson {
    fn new() -> anyhow::Result<Self> {
        let hub = Arc::new(IpcHub::new());
        let photon_rx = hub.register(boson_ipc::Process::Photon);
        let electron_rx = hub.register(boson_ipc::Process::Electron);

        let photon = Photon::new(hub.clone(), photon_rx);
        let electron = Electron::new(hub.clone(), electron_rx)?;

        Ok(Self {
            hub,
            photon,
            electron,
        })
    }

    pub fn start() -> anyhow::Result<()> {
        env_logger::init();

        let mut boson = Self::new()?;
        let mut photon = boson.photon;
        let mut electron = boson.electron;

        electron.start();

        eframe::run_native(
            "Photon",
            eframe::NativeOptions::default(),
            Box::new(|cc| {
                photon.init(cc, None, None, None);
                Ok(Box::new(photon))
            }),
        );

        Ok(())
    }
}
