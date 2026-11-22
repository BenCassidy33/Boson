use scraper::node::Element;
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum HtmlTag {
    // Document metadata
    Html,
    Head,
    Title,
    Base,
    Link,
    Meta,
    Style,

    // Document structure
    Body,
    Header,
    Footer,
    Main,
    Nav,
    Section,
    Article,
    Aside,
    Address,

    // Headings
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,

    // Text content
    Div,
    Span,
    P,
    Br,
    Hr,
    Pre,
    Blockquote,
    Figure,
    Figcaption,

    // Inline text semantics
    A,
    Em,
    Strong,
    Small,
    S,
    Cite,
    Q,
    Dfn,
    Abbr,
    Data,
    Time,
    Code,
    Var,
    Samp,
    Kbd,
    Sub,
    Sup,
    I,
    B,
    U,
    Mark,
    Ruby,
    Rt,
    Rp,
    Bdi,
    Bdo,
    Wbr,

    // Lists
    Ul,
    Ol,
    Li,
    Dl,
    Dt,
    Dd,
    Menu,

    // Images and embedded content
    Img,
    Picture,
    Source,
    Audio,
    Video,
    Track,
    Map,
    Area,
    Iframe,
    Embed,
    Object,
    Param,
    Canvas,
    Svg,
    Math,

    // Tables
    Table,
    Caption,
    Colgroup,
    Col,
    Thead,
    Tbody,
    Tfoot,
    Tr,
    Th,
    Td,

    // Forms
    Form,
    Label,
    Input,
    Button,
    Select,
    Datalist,
    Optgroup,
    Option,
    Textarea,
    Output,
    Progress,
    Meter,
    Fieldset,
    Legend,

    // Interactive elements
    Details,
    Summary,
    Dialog,

    // Scripting
    Script,
    Noscript,
    Template,
    Slot,

    // Web components
    #[strum(serialize = "search")]
    Search,

    // Obsolete / legacy elements
    Center,
    Font,
    Marquee,
    Strike,
    Big,
    Frame,
    Frameset,
    NoFrames,
    Acronym,
    Applet,
    Basefont,
    Dir,
    Isindex,
    Keygen,

    #[strum(default)]
    Custom(String),
}

impl HtmlTag {
    pub fn is_void(&self) -> bool {
        matches!(
            self,
            Self::Area
                | Self::Base
                | Self::Br
                | Self::Col
                | Self::Embed
                | Self::Hr
                | Self::Img
                | Self::Input
                | Self::Link
                | Self::Meta
                | Self::Param
                | Self::Source
                | Self::Track
                | Self::Wbr
        )
    }

    pub fn heading_level(&self) -> Option<u8> {
        match self {
            Self::H1 => Some(1),
            Self::H2 => Some(2),
            Self::H3 => Some(3),
            Self::H4 => Some(4),
            Self::H5 => Some(5),
            Self::H6 => Some(6),
            _ => None,
        }
    }

    pub fn from_element(element: &Element) -> Self {
        element
            .name()
            .parse()
            .unwrap_or(Self::Custom(element.name().to_string()))
    }
}
