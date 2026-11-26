#![allow(warnings)]

pub mod collider;
pub mod photon;
pub mod spectra;

pub use gluon;

use std::{any, sync::Arc};

use eframe::egui::mutex::Mutex;
use gluon::{GluonHub, GluonMessage};
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use scraper::Html;

use crate::{collider::Collider, photon::Photon, spectra::Spectra};

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum BosonProcess {
    Collider,
    Spectra,
    Photon,
}

pub struct Boson {
    hub: GluonHub<BosonProcess>,
}

#[derive(Debug, GluonMessage)]
pub enum GlobalIPCMessage {
    Shutdown,
}

impl Boson {
    pub fn start() -> anyhow::Result<Self> {
        env_logger::init();

        let hub = GluonHub::new();

        let collider = Collider::new()?;
        let spectra = Spectra::new(100)?;

        // hub.register(BosonProcesses::Photon, photon);
        hub.register(BosonProcess::Collider, collider);
        hub.register(BosonProcess::Spectra, spectra);

        let mut photon = Photon::new(&hub);

        eframe::run_native(
            "Photon",
            eframe::NativeOptions::default(),
            Box::new(|cc| {
                photon.init(cc, None, None, None);
                Ok(Box::new(photon))
            }),
        );

        Ok(Self { hub })
    }
}
