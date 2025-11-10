use std::error::Error;

slint::include_modules!();

pub fn start_application() -> Result<(), Box<dyn Error>> {
	let ui = App::new()?;

	ui.on_change_to_module_page(move |id: i32| -> Pages {
		Pages::from_id(id)
	});

	ui.run()?;
	Ok(())
}

impl Pages {
	pub fn from_id(id: i32) -> Self {
		println!("{}", id);
		match id {
			0 => Self::Menu,
			1 => Self::Button,
			2 => Self::Binary,
			3 => Self::ColorCode,
			4 => Self::Hexadecimal,
			5 => Self::Keypad,
			6 => Self::Mathematics,
			7 => Self::MorseCode,
			8 => Self::MultiButtons,
			9 => Self::Tiles,
			10 => Self::Timing,
			11 => Self::Wires,
			12 => Self::CountingNeedy,
			13 => Self::EchoNeedy,
			14 => Self::InterviewNeedy,
			_ => unreachable!(),
		}
	}
}
