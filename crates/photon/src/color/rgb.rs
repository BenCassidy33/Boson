use anyhow::bail;

use crate::color::Color;

#[derive(Debug)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<(u8, u8, u8)> for RGB {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}

impl RGB {
    pub const fn new(red: u8, blue: u8, green: u8) -> Self {
        Self { red, green, blue }
    }

    pub const fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub const fn white() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
        }
    }

    pub const fn red() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub const fn green() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub const fn blue() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 255,
        }
    }

    pub const fn yellow() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 0,
        }
    }

    pub const fn cyan() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 255,
        }
    }

    pub const fn magenta() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 255,
        }
    }

    pub const fn orange() -> Self {
        Self {
            red: 255,
            green: 165,
            blue: 0,
        }
    }

    pub const fn purple() -> Self {
        Self {
            red: 128,
            green: 0,
            blue: 128,
        }
    }

    pub const fn pink() -> Self {
        Self {
            red: 255,
            green: 192,
            blue: 203,
        }
    }

    pub const fn brown() -> Self {
        Self {
            red: 165,
            green: 42,
            blue: 42,
        }
    }

    pub const fn gray() -> Self {
        Self {
            red: 128,
            green: 128,
            blue: 128,
        }
    }

    pub const fn light_gray() -> Self {
        Self {
            red: 211,
            green: 211,
            blue: 211,
        }
    }

    pub const fn dark_gray() -> Self {
        Self {
            red: 64,
            green: 64,
            blue: 64,
        }
    }

    pub const fn silver() -> Self {
        Self {
            red: 192,
            green: 192,
            blue: 192,
        }
    }

    pub const fn navy() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 128,
        }
    }

    pub const fn teal() -> Self {
        Self {
            red: 0,
            green: 128,
            blue: 128,
        }
    }

    pub const fn lime() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub const fn olive() -> Self {
        Self {
            red: 128,
            green: 128,
            blue: 0,
        }
    }

    pub const fn maroon() -> Self {
        Self {
            red: 128,
            green: 0,
            blue: 0,
        }
    }

    pub const fn indigo() -> Self {
        Self {
            red: 75,
            green: 0,
            blue: 130,
        }
    }

    pub const fn violet() -> Self {
        Self {
            red: 238,
            green: 130,
            blue: 238,
        }
    }

    pub const fn gold() -> Self {
        Self {
            red: 255,
            green: 215,
            blue: 0,
        }
    }

    pub const fn coral() -> Self {
        Self {
            red: 255,
            green: 127,
            blue: 80,
        }
    }

    pub const fn salmon() -> Self {
        Self {
            red: 250,
            green: 128,
            blue: 114,
        }
    }

    pub fn try_parse(input: &str) -> anyhow::Result<Self> {
        if input.len() < 10 || !input.starts_with("rgb(") || !input.ends_with(")") {
            bail!("Invalid RGB color format");
        }

        let color_u8s = &input[4..input.len() - 1]
            .split(",")
            .map(|item| item.trim().parse::<u8>())
            .collect::<std::result::Result<Vec<u8>, _>>()?;

        if color_u8s.len() != 3 {
            bail!("Invalid RGB color format");
        }

        Ok(Self::from((color_u8s[0], color_u8s[1], color_u8s[2])))
    }
}

impl Color for RGB {
    fn to_hex(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    fn write_hex(&mut self, hex: u32) {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;

        self.red = r;
        self.green = g;
        self.blue = b;
    }
}
