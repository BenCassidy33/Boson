use eframe::egui;
use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet},
};
use scraper::Html;

use crate::{
    default_html, default_stylesheet,
    photon::{font_manager::FontManager, layout_engine::LayoutEngine},
    time_function,
};

pub mod element_display;
pub mod font_manager;
pub mod layout_engine;
pub mod tag;

#[derive(Debug)]
pub struct Photon<'a> {
    pub layout_engine: LayoutEngine<'a>,
    pub font_manager: FontManager,
    pub ctx: egui::Context,

    pub first_display: bool,
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

        time_function!("Fonts loaded in:", {
            font_manager.load_and_add_fonts(layout_engine.get_fonts(), cc);
        });

        Self {
            layout_engine,
            font_manager: FontManager::new(),
            ctx: cc.egui_ctx.clone(),

            first_display: false,
        }
    }

    pub fn start(
        cc: &eframe::CreationContext<'_>,
    ) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
        let photon = time_function!("Photon started in:", { Photon::new(cc, None, None, None) });
        Ok(Box::new(photon))
    }
}

impl<'a> eframe::App for Photon<'a> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if !self.first_display {
                time_function!(
                    "First Render in: ",
                    self.layout_engine.display(ui, &self.font_manager).unwrap()
                );

                self.first_display = true;
            } else {
                self.layout_engine.display(ui, &self.font_manager).unwrap();
            }
        });
    }
}
