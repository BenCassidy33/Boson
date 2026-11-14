use boson_window::BosonWindow;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{self, ControlFlow, EventLoop};
use winit::window::Window;

// #[path = "../examples/utils/winit_app.rs"]
// mod winit_app;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut window = BosonWindow::new(&event_loop).unwrap();
    window.run(event_loop);
}
