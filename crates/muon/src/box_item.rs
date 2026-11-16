use photon::color::Color;
use scraper::ElementRef;

pub struct Edges {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

pub enum BorderKind {
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    None,
    Hidden,
}

pub struct BorderItem<'a> {
    width: u32,
    border_kind: BorderKind,
    color: &'a dyn Color,
}

pub struct Border<'a> {
    top: BorderItem<'a>,
    right: BorderItem<'a>,
    bottom: BorderItem<'a>,
    left: BorderItem<'a>,
}

pub struct BoxItem<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    pub margin: Edges,
    pub padding: Edges,
    pub border: Border<'a>,

    pub element: ElementRef<'a>,
    pub content: Vec<BoxItem<'a>>,
}
