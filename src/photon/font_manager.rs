use std::{collections::HashMap, fs, os::fd};

use eframe::{
    CreationContext,
    egui::{FontData, FontInsert},
};
use lightningcss::{printer::PrinterOptions, properties::font::FontFamily, traits::ToCss};
use log::{info, warn};

#[derive(Debug)]
pub struct FontManager {
    pub loaded_fonts: HashMap<String, fontdb::ID>,
    pub database: fontdb::Database,
}

impl FontManager {
    pub fn new() -> Self {
        let mut database = fontdb::Database::default();
        database.load_system_fonts();

        Self {
            loaded_fonts: HashMap::new(),
            database,
        }
    }

    pub fn load_fonts(&mut self, fonts: Vec<&FontFamily<'_>>) {
        for font in fonts {
            self.load_font(font);
        }
    }

    pub fn load_font(&mut self, font: &FontFamily<'_>) {
        let font_name = match font {
            FontFamily::Generic(generic_font_family) => {
                let Ok(font_name) = generic_font_family.to_css_string(PrinterOptions::default())
                else {
                    warn!("Failed to convert to css_string");
                    return;
                };

                font_name
            }

            FontFamily::FamilyName(family_name) => {
                let Ok(font_name) = family_name.to_css_string(PrinterOptions::default()) else {
                    warn!("Failed to convert to css_string");
                    return;
                };

                font_name
            }
        };

        let family = fontdb::Family::Name(&font_name);
        let query = fontdb::Query {
            families: &[family],
            ..Default::default()
        };

        let Some(font_id) = self.database.query(&query) else {
            warn!("Failed to get font path for font: {}", font_name);
            return;
        };

        self.loaded_fonts.insert(font_name, font_id);
    }

    pub fn add_fonts(&self, cc: &CreationContext<'_>) {
        for (font_name, font_id) in &self.loaded_fonts {
            let Some(face) = self.database.face(*font_id) else {
                warn!("Failed to get font face for {}", font_name);
                continue;
            };

            let font_data = match &face.source {
                fontdb::Source::Binary(as_ref) | fontdb::Source::SharedFile(_, as_ref) => {
                    FontData::from_blob(as_ref.clone(), face.index)
                }
                fontdb::Source::File(path_buf) => {
                    let Ok(data) = fs::read(&path_buf) else {
                        warn!(
                            "Failed to get font byte data for {} at {}",
                            font_name,
                            path_buf.display()
                        );
                        return;
                    };

                    FontData::from_owned(data)
                }
            };

            let new_font = FontInsert::new(&font_name, font_data, Vec::new());
            cc.egui_ctx.add_font(new_font);
            info!("Successfully loaded font: {}", font_name);
        }
    }
}
