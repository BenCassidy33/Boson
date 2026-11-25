#![allow(warnings)]

pub mod collider;
pub mod photon;
pub mod spectra;

use std::{any, sync::Arc};

use boson_macros::BosonIpcMessage;
use eframe::egui::mutex::Mutex;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use scraper::Html;

use crate::{collider::Collider, photon::Photon, spectra::Spectra};
use boson_ipc::{IpcHub, IpcMessage, ProcessRecv};

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
    spectra: Spectra,
    collider: Collider,
}

#[derive(Debug, BosonIpcMessage)]
pub enum GlobalIPCMessage {
    Shutdown,
}

impl Boson {
    pub fn new() -> anyhow::Result<Self> {
        let hub = Arc::new(IpcHub::new());
        let photon_rx = hub.register(boson_ipc::Process::Photon);
        let collider_rx = hub.register(boson_ipc::Process::Collider);
        let spectra_rx = hub.register(boson_ipc::Process::Spectra);

        let photon = Photon::new(hub.clone(), photon_rx);
        let collider = Collider::new(hub.clone(), collider_rx)?;
        let spectra = Spectra::new(hub.clone(), spectra_rx, 100)?;

        Ok(Self {
            hub,
            photon,
            spectra,
            collider,
        })
    }

    pub fn start() -> anyhow::Result<()> {
        Self::start_with(Boson::new()?)
    }

    pub fn start_with(boson: Boson) -> anyhow::Result<()> {
        env_logger::init();

        let mut photon = boson.photon;
        let mut collider = boson.collider;
        let mut spectra = boson.spectra;

        collider.start();
        spectra.start();

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
