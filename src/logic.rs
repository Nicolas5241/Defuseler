use std::error::Error;

slint::include_modules!();

pub fn start_application() -> Result<(), Box<dyn Error>> {
	let ui = App::new()?;

	ui.run()?;
	Ok(())
}
