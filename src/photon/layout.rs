use std::fmt::Debug;

use eframe::{
    CreationContext,
    egui::{self},
};
use lightningcss::{
    properties::{Property, font::FontFamily},
    rules::CssRule,
    stylesheet::{ParserOptions, StyleSheet},
};
use scraper::{Html, Node, Selector, html::Select};

use crate::{DEFAULT_HTML, DEFAULT_STYLESHEET};

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

    pub fn select<'b>(&'b self, selector: &'b Selector) -> Select<'b, '_> {
        self.document.select(selector)
    }

    pub fn parse(html: &str) -> Html {
        if html.starts_with("<!doctype") {
            return Html::parse_document(html);
        }

        Html::parse_fragment(html)
    }

    pub fn update_layout(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn display(&self, ui: eframe::egui::Ui, node: Node) {
        match node {
            Node::Doctype(doctype) => todo!(),
            Node::Text(text) => todo!(),
            Node::Element(element) => todo!(),
            _ => todo!(),
        }
    }
}
