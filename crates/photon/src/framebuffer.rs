use std::{num::NonZeroU32, sync::Arc};

use softbuffer::{Buffer, Surface};
use winit::{
    event_loop::OwnedDisplayHandle,
    window::{self, Window},
};

use crate::color::Color;

pub struct Framebuffer {
    pixels: Vec<u32>,
    width: NonZeroU32,
    height: NonZeroU32,
}

impl Framebuffer {
    pub fn new(width: NonZeroU32, height: NonZeroU32) -> Self {
        let size = width.get() as usize * height.get() as usize;
        Self {
            width,
            height,
            pixels: vec![0; size],
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: &dyn Color) {
        if x >= self.width.into() || y >= self.height.into() {
            return;
        }

        self.pixels[(y * self.width.get() + x) as usize] = color.to_hex();
    }

    pub fn clear(&mut self, color: &dyn Color) {
        self.pixels.fill(color.to_hex());
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn width(&self) -> NonZeroU32 {
        self.width
    }

    pub fn height(&self) -> NonZeroU32 {
        self.height
    }
}

pub struct DoubleBuffer {
    front: Framebuffer,
    back: Framebuffer,

    width: NonZeroU32,
    height: NonZeroU32,
}

impl DoubleBuffer {
    pub fn new(width: NonZeroU32, height: NonZeroU32) -> Self {
        Self {
            front: Framebuffer::new(width, height),
            back: Framebuffer::new(width, height),
            width,
            height,
        }
    }

    pub fn front(&self) -> &Framebuffer {
        &self.front
    }

    pub fn back(&mut self) -> &mut Framebuffer {
        &mut self.back
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }

    pub fn width(&self) -> NonZeroU32 {
        self.width
    }

    pub fn height(&self) -> NonZeroU32 {
        self.height
    }

    pub fn clear(&mut self, color: &dyn Color) {
        self.back.clear(color);
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: &dyn Color) {
        if x >= self.width.into() || y >= self.height.into() {
            return;
        }

        self.back.pixels[(y * self.width.get() + x) as usize] = color.to_hex();
    }

    pub fn resize(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.front = Framebuffer::new(width, height);
        self.back = Framebuffer::new(width, height);

        self.width = width;
        self.height = height;
    }

    pub fn front_pixels(&self) -> &[u32] {
        self.front.pixels()
    }

    pub fn back_pixels(&self) -> &[u32] {
        self.back.pixels()
    }

    pub fn present(
        &self,
        surface: &mut Surface<OwnedDisplayHandle, Arc<Window>>,
    ) -> anyhow::Result<()> {
        let mut buffer = surface.buffer_mut().unwrap();

        buffer.copy_from_slice(self.front.pixels());
        buffer.present().unwrap();

        Ok(())
    }
}
