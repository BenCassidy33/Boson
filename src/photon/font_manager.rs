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

    pub fn load_and_add_fonts(&mut self, font_families: Vec<&FontFamily<'_>>, cc: &CreationContext<'_>) {
        for family in font_families {
            self.load_and_add_font(family, cc);
        }
    }

    pub fn load_and_add_font(&mut self, font_family: &FontFamily<'_>, cc: &CreationContext<'_>) {
        self.load_font(font_family);

        let Some(font_name) = Self::get_font_name(font_family) else {
            return;
        };
        let Some(font_id) = self.get_font_id_from_font_family(font_family) else {
            return;
        };
        self.add_font(&font_name, &font_id, cc);
    }

    pub fn get_font_name(font_family: &FontFamily<'_>) -> Option<String> {
        match font_family {
            FontFamily::Generic(generic_font_family) => {
                let Ok(font_name) = generic_font_family.to_css_string(PrinterOptions::default())
                else {
                    warn!("Failed to convert to css_string");
                    return None;
                };

                Some(font_name)
            }

            FontFamily::FamilyName(family_name) => {
                let Ok(font_name) = family_name.to_css_string(PrinterOptions::default()) else {
                    warn!("Failed to convert to css_string");
                    return None;
                };

                Some(font_name)
            }
        }
    }

    pub fn get_font_id(&self, font_family: String) -> Option<fontdb::ID> {
        let family = fontdb::Family::Name(&font_family);
        let query = fontdb::Query {
            families: &[family],
            ..Default::default()
        };
        self.database.query(&query)
    }

    pub fn get_font_id_from_font_family(&self, font_family: &FontFamily<'_>) -> Option<fontdb::ID> {
        let Some(font_name) = Self::get_font_name(font_family) else {
            return None;
        };

        self.get_font_id(font_name)
    }

    pub fn load_font(&mut self, font_family: &FontFamily<'_>) {
        let Some(font_name) = Self::get_font_name(font_family) else {
            return;
        };

        let Some(font_id) = self.get_font_id_from_font_family(&font_family) else {
            return;
        };

        self.loaded_fonts.insert(font_name, font_id);
    }

    pub fn add_loaded_fonts(&self, cc: &CreationContext<'_>) {
        for (font_name, font_id) in &self.loaded_fonts {
            self.add_font(font_name, font_id, cc);
        }
    }

    pub fn add_font(&self, font_name: &String, font_id: &fontdb::ID, cc: &CreationContext<'_>) {
        let Some(face) = self.database.face(*font_id) else {
            warn!("Failed to get font face for {}", font_name);
            return;
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
