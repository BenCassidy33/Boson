#![allow(warnings)]

use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use scraper::Html;

pub mod photon;

pub const DEFAULT_HTML: &str = "<!doctype HTML><html><head></head><body></body></html>";
pub const DEFAULT_STYLESHEET: &str = include_str!("../resources/default_styles.css");

#[inline]
pub fn default_html() -> Html {
    Html::parse_document(DEFAULT_HTML)
}

#[inline]
pub fn default_stylesheet<'a>(opts: Option<ParserOptions<'a>>) -> StyleSheet<'a> {
    StyleSheet::parse(DEFAULT_STYLESHEET, opts.unwrap_or_default())
        .expect("Failed to parse default stylesheet")
}
