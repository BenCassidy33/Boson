use eframe::egui;
use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet},
};
use scraper::Html;

use crate::{
    default_html, default_stylesheet,
    photon::{font_manager::FontManager, layout::LayoutEngine},
};

pub mod font_manager;
pub mod layout;

#[derive(Debug)]
pub struct Photon<'a> {
    pub layout_engine: LayoutEngine<'a>,
    pub font_manager: FontManager,
}

impl<'a> Photon<'a> {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        document: Option<Html>,
        stylesheet: Option<StyleSheet<'a>>,
        opts: Option<ParserOptions<'a>>,
    ) -> Self {
        let document = document.unwrap_or(default_html());
        let stylesheet = stylesheet.unwrap_or(default_stylesheet(opts));
        let layout_engine = LayoutEngine::new(document, stylesheet, cc);
        let mut font_manager = FontManager::new();

        let fonts = layout_engine.get_fonts();
        font_manager.load_fonts(fonts);
        font_manager.add_fonts(cc);

        Self {
            layout_engine,
            font_manager: FontManager::new(),
        }
    }
}

impl<'a> eframe::App for Photon<'a> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.layout_engine.update_layout();
        });
    }
}
