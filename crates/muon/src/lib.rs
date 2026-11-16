pub mod box_item;

use std::borrow::Cow;

use lightningcss::{
    rules::{CssRule, CssRuleList, style::StyleRule},
    stylesheet::{ParserOptions, StyleSheet},
};
use scraper::{ElementRef, Html, Selector, html::Select};

const DOCUMENT_PREFIX: &str = "<!doctype html>";

#[derive(Debug)]
pub struct LayoutEngine<'a> {
    html: Option<Html>,
    styles: CssRuleList<'a>,
}

impl<'a> LayoutEngine<'a> {
    pub fn new() -> Self {
        Self {
            html: None,
            styles: CssRuleList(Vec::new()),
        }
    }

    pub fn parse_html(&mut self, input: &str) -> Result<(), Vec<Cow<'static, str>>> {
        if input.len() > DOCUMENT_PREFIX.len()
            && input[0..DOCUMENT_PREFIX.len()].to_ascii_lowercase() == DOCUMENT_PREFIX
        {
            return self.parse_document(input);
        }

        self.parse_fragment(input)
    }

    fn parse_document(&mut self, document: &str) -> Result<(), Vec<Cow<'static, str>>> {
        let html = Html::parse_document(document);

        if !html.errors.is_empty() {
            return Err(html.errors);
        }

        self.html = Some(html);

        Ok(())
    }

    fn parse_fragment(&mut self, fragment: &str) -> Result<(), Vec<Cow<'static, str>>> {
        let html = Html::parse_fragment(fragment);

        if !html.errors.is_empty() {
            return Err(html.errors);
        }

        self.html = Some(html);

        Ok(())
    }

    pub fn select<'e, 'b>(
        &'e self,
        selector: &'b Selector,
    ) -> anyhow::Result<Option<Select<'e, 'b>>> {
        let Some(html) = self.html.as_ref() else {
            return Ok(None);
        };

        Ok(Some(html.select(selector)))
    }

    pub fn parse_stylesheet(&mut self, mut stylesheet: &'a str) -> Result<(), ()> {
        let stylesheet = StyleSheet::parse(stylesheet, ParserOptions::default());
        if stylesheet.is_err() {
            return Err(());
        }

        self.styles.0.append(&mut stylesheet.unwrap().rules.0);

        Ok(())
    }

    pub fn add_style_rule(&mut self, rule: CssRule<'a>) {
        self.styles.0.push(rule);
    }

    pub fn add_stylesheet(&mut self, mut stylesheet: StyleSheet<'a>) {
        self.styles.0.append(&mut stylesheet.rules.0);
    }

    pub fn get_elements_for(&self, selector: &Selector) -> Option<Vec<ElementRef>> {
        todo!()
    }

    pub fn get_styles_for(&self, element_ref: ElementRef<'a>) -> Option<Vec<StyleRule>> {
        if self.styles.0.is_empty() {
            return None;
        }

        for rule in &self.styles.0 {
            let CssRule::Style(style_rule) = rule else {
                continue;
            };

            dbg!(style_rule);
        }

        todo!()
    }
}

impl<'a> Default for LayoutEngine<'a> {
    fn default() -> Self {
        Self::new()
    }
}
