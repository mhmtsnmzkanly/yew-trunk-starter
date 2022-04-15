pub mod utils;
pub mod app;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<crate::app::App>();
    log::info!("Yew App is ready.");
}
