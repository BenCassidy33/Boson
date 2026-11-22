use boson::photon::{self, Photon, layout::LayoutEngine};

use scraper::Selector;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_timing::Histogram;

fn init_tracing() {
    let timing_layer = tracing_timing::Builder::default()
        .layer(|| Histogram::new_with_max(60_000_000_000, 3).unwrap());

    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_filter(
                    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
                ),
        )
        .with(timing_layer);

    subscriber.init();
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    // init_tracing();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Photon",
        native_options,
        Box::new(|cc| {
            let photon = Photon::new(cc, None, None, None);
            Ok(Box::new(photon))
        }),
    )?;

    Ok(())
}
