use std::{fmt::Debug, sync::LazyLock};

use anyhow::Context;
use eframe::{
    CreationContext,
    egui::{self, Response},
};
use lightningcss::{
    properties::{Property, font::FontFamily},
    rules::CssRule,
    stylesheet::{ParserOptions, StyleSheet},
};
use log::warn;
use scraper::{ElementRef, Html, Node, Selector, html::Select};

use crate::{
    DEFAULT_HTML, DEFAULT_STYLESHEET,
    photon::{element_display, font_manager::FontManager},
};

pub struct LayoutEngine<'a> {
    document: Html,
    stylesheet: StyleSheet<'a>,
    ctx: Option<egui::Context>,
}

impl Debug for LayoutEngine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutEngine")
            .field("document", &self.document)
            .field("stylesheet", &self.stylesheet)
            .finish()
    }
}

impl<'a> Default for LayoutEngine<'a> {
    fn default() -> Self {
        let stylesheet = StyleSheet::parse(DEFAULT_STYLESHEET, ParserOptions::default())
            .expect("Failed to parse default stylesheet!");

        Self {
            document: Self::parse(DEFAULT_HTML),
            stylesheet,
            ctx: None,
        }
    }
}

impl<'a> LayoutEngine<'a> {
    pub fn new(document: Html, stylesheet: StyleSheet<'a>, ctx: &CreationContext<'_>) -> Self {
        let s = Self {
            document,
            stylesheet,
            ctx: Some(ctx.egui_ctx.clone()),
        };

        s.init();

        s
    }

    fn init(&self) {
        self.setup_imports();
    }

    fn setup_imports(&self) {}

    pub fn get_fonts<'b>(&'b self) -> Vec<&'b FontFamily> {
        let rule_list = &self.stylesheet.rules.0;
        let mut selected_fonts = Vec::new();

        for rule in rule_list {
            match rule {
                CssRule::Style(style) => {
                    for prop in &style.declarations.declarations {
                        if let Property::Font(fonts) = &prop {
                            for font in fonts.family.iter() {
                                selected_fonts.push(font);
                            }
                        }

                        if let Property::FontFamily(fonts) = &prop {
                            for font in fonts {
                                selected_fonts.push(font);
                            }
                        }
                    }
                }
                _ => continue,
            }
        }

        selected_fonts
    }

    fn load_font() {
        todo!()
    }

    pub fn select<'s, 'b>(&'s self, selector: &'b Selector) -> Select<'s, 'b> {
        self.document.select(selector)
    }

    pub fn parse(html: &str) -> Html {
        if html.starts_with("<!doctype") {
            return Html::parse_document(html);
        }

        Html::parse_fragment(html)
    }

    // pub fn update_layout(&mut self) -> anyhow::Result<()> {
    //     Ok(())
    // }
    //
    pub fn get_head_info(&self) {
        todo!()
    }

    pub fn get_body(&self) -> anyhow::Result<ElementRef<'_>> {
        let selector = Selector::parse("body")?;
        let body = self
            .select(&selector)
            .next()
            .context("Failed to get body for document!")?;

        Ok(body)
    }

    pub fn display(
        &self,
        ui: &mut eframe::egui::Ui,
        font_manager: &FontManager,
    ) -> anyhow::Result<()> {
        let body = self.get_body()?;
        if !body.has_children() {
            warn!("HTML Body is empty!");
            return Ok(());
        }

        for child in body.child_elements() {
            element_display::display(ui, *child, &self.stylesheet, font_manager);
        }

        Ok(())
    }
}
