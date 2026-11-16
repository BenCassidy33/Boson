pub mod rgb;

use anyhow::bail;

pub trait Color {
    fn to_hex(&self) -> u32;
    fn write_hex(&mut self, hex: u32);
}
