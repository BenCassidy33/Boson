use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use muon::LayoutEngine;
use scraper::Selector;

fn main() {
    let mut engine = LayoutEngine::new();

    engine.parse_html("<!DOCTYPE html><html><div id='test-el'>Hello World!</div></html>");

    let stylesheet =
        StyleSheet::parse("#test-el { display: flex; }", ParserOptions::default()).unwrap();

    dbg!(&stylesheet.rules);
    engine.add_stylesheet(stylesheet);

    let selector = Selector::parse("#test-el").unwrap();
    engine.select(selector)

    // dbg!(result);
}
