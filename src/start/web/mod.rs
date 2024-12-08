#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
/// The starting point when loaded via wasm bindgen
fn wasm_start() -> Result<(), JsValue> {
    use crate::{build, setup};

    // Temporary panic hook until logger is finished initializing
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    // Transfers embedded resources into utility crate
    utils::resources::store_embedded_resources(crate::resources::get_embedded_resources());

    // Helps with license compliance
    build::license_compliance_helper();

    debug!("Launched as Wasm library...");
    build::log_build_info();

    if let Err(error) = setup::start() {
        error!("{}", error);

        return Err(JsValue::from(error));
    }

    Ok(())
}
