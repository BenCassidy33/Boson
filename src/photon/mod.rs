use std::sync::Arc;

use boson_ipc::{IpcHub, IpcMessageExt, IpcPayload, ProcessRecv};
use eframe::egui;
use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet},
};
use scraper::Html;

use crate::{
    collider::Collider,
    default_html, default_stylesheet,
    photon::{
        self, font_manager::FontManager, layout_engine::LayoutEngine, photon_ipc::FetchResource,
    },
    spectra::spectra_ipc::{ReplyFrameData, RequestFrameData},
    time_function,
};

pub mod element_display;
pub mod font_manager;
pub mod layout_engine;
pub mod photon_ipc;
pub mod statistics;
pub mod tag;

pub struct Photon<'a> {
    pub ipc_hub: Arc<IpcHub>,
    pub recv: ProcessRecv,
    pub font_manager: FontManager,
    pub first_display: bool,
    pub layout_engine: Option<LayoutEngine<'a>>,
    pub ctx: Option<egui::Context>,
}

impl<'a> Photon<'a> {
    pub fn new(ipc_hub: Arc<IpcHub>, recv: ProcessRecv) -> Self {
        Self {
            ipc_hub,
            recv,
            font_manager: FontManager::new(),
            first_display: false,
            layout_engine: None,
            ctx: None,
        }
    }

    pub fn init(
        &mut self,
        cc: &eframe::CreationContext<'_>,
        document: Option<Html>,
        stylesheet: Option<StyleSheet<'a>>,
        opts: Option<ParserOptions<'a>>,
    ) {
        let document = document.unwrap_or(default_html());
        let stylesheet = stylesheet.unwrap_or(default_stylesheet(opts));

        self.ctx = Some(cc.egui_ctx.clone());
        self.layout_engine = Some(LayoutEngine::new(document, stylesheet, cc));

        time_function!("Fonts loaded in:", {
            self.font_manager
                .load_and_add_fonts(self.layout_engine.as_ref().unwrap().get_fonts(), cc);
        });
    }

    pub fn layout_engine(&self) -> &'a LayoutEngine {
        self.layout_engine
            .as_ref()
            .expect("Photon Not Yet Initalized!")
    }
}

// TODO: This needs to be async somehow
impl<'a> eframe::App for Photon<'a> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        self.ipc_hub
            .send(boson_ipc::Process::Spectra, RequestFrameData {});

        egui::CentralPanel::default().show(ui, |ui| {
            if let Ok(msg) = self.recv.try_recv() {
                todo!();
            }

            if !self.first_display {
                time_function!(
                    "First Render in: ",
                    self.layout_engine
                        .as_ref()
                        .unwrap()
                        .display(ui, &self.font_manager)
                        .unwrap()
                );

                self.first_display = true;
            } else {
                self.layout_engine
                    .as_ref()
                    .unwrap()
                    .display(ui, &self.font_manager)
                    .unwrap();
            }
        });
    }
}
