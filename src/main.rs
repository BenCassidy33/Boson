use boson::photon::{self, Photon, layout::LayoutEngine};
use scraper::Selector;

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Photon",
        native_options,
        Box::new(|cc| {
            let photon = Photon::new(cc, None, None, None);
            Ok(Box::new(photon))
        }),
    );
}
