use photon::PhotonWindow;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut window = PhotonWindow::new(&event_loop).unwrap();
    let _ = window.run(event_loop);
}
