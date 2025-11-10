#![cfg(target_os = "android")]

use std::error::Error;
mod logic;
mod bomb_modules;
mod types;

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn Error>>{
	slint::android::init(app)?;

	logic::start_application()
}
