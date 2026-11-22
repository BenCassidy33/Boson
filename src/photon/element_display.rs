use eframe::egui::{self, Response, RichText};
use ego_tree::NodeRef;
use lightningcss::{
    declaration::DeclarationBlock,
    properties::{Property, PropertyId},
    stylesheet::{ParserOptions, StyleSheet},
    traits::Parse,
};
use scraper::{ElementRef, node::Element};

use crate::photon::{font_manager::FontManager, tag::HtmlTag};

fn parse_all_inline_styles(styles: &str) -> anyhow::Result<Vec<Property<'_>>> {
    styles
        .split(";")
        .filter(|style| !style.trim().is_empty())
        .map(parse_inline_style)
        .collect()
}

fn parse_inline_style(style: &str) -> anyhow::Result<Property<'_>> {
    let (p_id, p_val) = style
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("Expected ':' in style: {style:?}"))?;

    let id = PropertyId::parse_string(p_id.trim())
        .map_err(|e| anyhow::anyhow!("Failed to parse property ID: {e:?}"))?;

    let property = Property::parse_string(id, p_val.trim(), ParserOptions::default())
        .map_err(|e| anyhow::anyhow!("Failed to parse property value: {e:?}"))?;

    Ok(property)
}

pub fn display(
    ui: &mut eframe::egui::Ui,
    node_ref: NodeRef<'_, scraper::Node>,
    stylesheet: &StyleSheet<'_>,
    font_manager: &FontManager,
) -> anyhow::Result<Response> {
    match node_ref.value() {
        scraper::Node::Text(text) => {
            let mut rt = RichText::new(text.to_string());
            if let Some(font_id) = font_manager.main_font {
                rt.font(font_manager.get_main_font_for_egui(FontManager::DEFAULT_FONT_SIZE));
            };

            Ok(ui.label(text.to_string()))
        }

        scraper::Node::Element(element) => {
            display_element(ui, node_ref, element, stylesheet, font_manager)
        }

        todo => todo!("{:?}", todo),
    }
}

fn display_element(
    ui: &mut eframe::egui::Ui,
    node_ref: NodeRef<'_, scraper::Node>,
    element: &Element,
    stylesheet: &StyleSheet<'_>,
    font_manager: &FontManager,
) -> anyhow::Result<Response> {
    let id = element.id();
    let classes = element.classes().collect::<Vec<_>>();
    let tag = HtmlTag::from_element(element);
    let styles = element
        .attr("style")
        .map(parse_all_inline_styles)
        .transpose()?
        .unwrap_or_default();

    match tag {
        HtmlTag::Div => Ok(egui::Frame::NONE
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // apply_styles_for(ui, node_ref, element, stylesheet, font_manager);

                    for child in node_ref.children() {
                        if let scraper::Node::Text(text) = child.value()
                            && text.trim().is_empty()
                        {
                            continue;
                        }

                        let _ = display(ui, child, stylesheet, font_manager);
                    }
                })
            })
            .response),

        tag => todo!("{:?}", tag),
    }
}

fn apply_styles_for(
    ui: &mut eframe::egui::Ui,
    node_ref: NodeRef<'_, scraper::Node>,
    element: &Element,
    stylesheet: &StyleSheet<'_>,
    font_manager: &FontManager,
) {
    todo!()
}
