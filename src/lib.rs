#![cfg(target_os = "android")]

use std::error::Error;
mod logic;

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn Error>>{
	slint::android::init(app)?;

	logic::start_application()
}
