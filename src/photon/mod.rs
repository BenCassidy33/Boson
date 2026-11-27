use std::sync::Arc;

use eframe::egui;
use gluon::{
    GluonHub,
    ctx::GluonProcessContext,
    proc::{GluonError, GluonProcess},
};
use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet},
};
use scraper::Html;

use crate::{
    TachyonProcess,
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
    pub font_manager: FontManager,
    pub first_display: bool,
    pub layout_engine: Option<LayoutEngine<'a>>,
    pub ctx: Option<egui::Context>,
    pub hub: &'a GluonHub<TachyonProcess>,
}

impl<'a> Photon<'a> {
    pub fn new(hub: &'a GluonHub<TachyonProcess>) -> Self {
        Self {
            font_manager: FontManager::new(),
            first_display: false,
            layout_engine: None,
            ctx: None,
            hub,
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

impl<'a> eframe::App for Photon<'a> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
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
