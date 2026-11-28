use lightningcss::{
    declaration::DeclarationBlock,
    printer::PrinterOptions,
    rules::{CssRule, style::StyleRule},
    stylesheet::StyleSheet,
    traits::ToCss,
};
use scraper::{ElementRef, Selector, node::Element};

pub struct StyleManager<'css> {
    pub stylesheet: StyleSheet<'css>,
}

impl<'css> StyleManager<'css> {
    pub fn new(stylesheet: StyleSheet<'css>) -> Self {
        Self { stylesheet }
    }

    pub fn get_styles_for<'dom>(&self, element: &ElementRef<'dom>) -> Vec<ComputedStyle> {
        let rules = &self.stylesheet.rules.0;
        let mut found_rules = Vec::new();

        for rule in rules {
            let CssRule::Style(style_rule) = rule else {
                continue;
            };

            let selectors = &style_rule.selectors.0;

            for selector in selectors {
                let selector_string = selector.to_css_string(PrinterOptions::default()).unwrap();
                let Ok(scraper_selector) = Selector::parse(&selector_string) else {
                    continue;
                };

                if scraper_selector.matches(element) {
                    let computed = ComputedStyle::from(style_rule);
                    found_rules.push(computed);
                }
            }
        }

        found_rules
    }

    pub fn apply_declerations_for<'dom>(
        ui: &mut eframe::egui::Ui,
        element: &ElementRef<'dom>,
        declerations: &DeclarationBlock<'css>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ComputedStyle {}

impl From<&StyleRule<'_>> for ComputedStyle {
    fn from(value: &StyleRule<'_>) -> Self {
        todo!()
    }
}

impl ComputedStyle {
    pub fn apply_to(&self, ui: &mut eframe::egui::Ui) {
        todo!()
    }
}
