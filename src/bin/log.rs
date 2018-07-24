#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init();
    log_enabled!(log::Level::Info);
    trace!("プリントスタックトレース");
    debug!("デバッグすんぞ");
    info!("インフォメーションテクノロジー");
    warn!("Warning Warning");
    error!("Error Error");
}