#![cfg(target_os = "android")]

use winit::platform::android::activity::AndroidApp;

#[unsafe(no_mangle)]
#[cfg(feature = "client")]
/// The starting point when loaded as an Android app
pub fn android_main(app: AndroidApp) {
    use crate::{build, setup};

    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    // Transfers embedded resources into utility crate
    utils::resources::store_embedded_resources(crate::resources::get_embedded_resources());

    // Helps with license compliance
    build::license_compliance_helper();

    debug!("Launched as Android app...");
    build::log_build_info();

    client::game::store_android_app(app);
    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}
