#![allow(warnings)]

pub mod color;
pub mod framebuffer;

use softbuffer::{Context as SoftbufferContext, Surface as SoftbufferSurface};
use std::{
    num::NonZeroU32,
    sync::{
        Arc,
        mpsc::{self, Receiver, Sender},
    },
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle},
    raw_window_handle::{HasRawWindowHandle, HasWindowHandle},
    window::{Window as WinitWindow, WindowAttributes},
};

use crate::{
    color::{BosonColor, RGB},
    framebuffer::{DoubleBuffer, Framebuffer},
};

pub struct BosonWindow {
    window: Option<Arc<WinitWindow>>,
    surface: Option<SoftbufferSurface<OwnedDisplayHandle, Arc<WinitWindow>>>,
    buffers: Option<DoubleBuffer>,

    context: SoftbufferContext<OwnedDisplayHandle>,
    attributes: WindowAttributes,
    sender: Sender<BosonWindowAction>,
    receiver: Receiver<BosonWindowAction>,
}

impl BosonWindow {
    pub fn new(event_loop: &EventLoop<()>) -> anyhow::Result<Self> {
        Self::with_attributes(event_loop, WinitWindow::default_attributes())
    }

    pub fn with_attributes(
        event_loop: &EventLoop<()>,
        attributes: WindowAttributes,
    ) -> anyhow::Result<Self> {
        // event_loop.set_control_flow(ControlFlow::Wait);

        let (sender, receiver) = mpsc::channel();
        let context = SoftbufferContext::new(event_loop.owned_display_handle()).unwrap();

        Ok(Self {
            window: None,
            surface: None,
            buffers: None,

            context,
            attributes,
            sender,
            receiver,
        })
    }

    pub fn run(&mut self, event_loop: EventLoop<()>) -> anyhow::Result<()> {
        Ok(event_loop.run_app(self)?)
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) -> anyhow::Result<()> {
        let window = Arc::new(event_loop.create_window(self.attributes.clone())?);
        let surface = SoftbufferSurface::new(&self.context, window.clone())
            .expect("Failed to create window surface!");

        let inner_size = window.inner_size();
        let buffers = DoubleBuffer::new(
            NonZeroU32::new(inner_size.width).unwrap(),
            NonZeroU32::new(inner_size.height).unwrap(),
        );

        self.window = Some(window);
        self.surface = Some(surface);
        self.buffers = Some(buffers);

        Ok(())
    }

    fn clear(&mut self, color: &dyn BosonColor) {
        self.buffers.as_mut().unwrap().clear(color);
    }

    fn draw_buffer(&mut self) {
        let Some(surface) = self.surface.as_mut() else {
            return;
        };

        let Some(buffers) = self.buffers.as_mut() else {
            return;
        };

        let Some(window) = self.window.as_ref() else {
            return;
        };

        let size = window.inner_size();

        if size.width == 0 || size.height == 0 {
            return;
        }

        let width = NonZeroU32::new(size.width).unwrap();
        let height = NonZeroU32::new(size.height).unwrap();

        surface.resize(width, height).unwrap();

        if buffers.width() != width || buffers.height() != height {
            buffers.resize(width, height);
        }

        buffers.clear(&RGB::blue());
        buffers.swap();
        buffers.present(surface);

        window.request_redraw();
    }
}

impl ApplicationHandler for BosonWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(error) = self.create_window(event_loop) {
            eprintln!("Failed to create window: {error}");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.draw_buffer();
            }

            _ => {}
        }
    }
}

pub enum BosonWindowAction {}
