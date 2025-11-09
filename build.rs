use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	slint_build::compile("ui/App.slint")?;

	Ok(())
}
