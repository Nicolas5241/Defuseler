use slint::{Model, ModelRc, SharedString, VecModel, Weak};
use crate::{logic::App, types::{ButtonText, Color}};

pub fn solve(ui_handle: Weak<App>, color: Color, text: ButtonText) {
	let ui = ui_handle.unwrap();

	let press_times: Option<u8>;
	let press_times_str: SharedString;
	let direction: SharedString;

	if color == Color::Blue && text == ButtonText::Detonate {
		press_times = Some(1);
	} else if color == Color::Red {
		press_times = Some(2);
	} else if text == ButtonText::Abort {
		press_times = Some(3);
	} else if color == Color::White {
		press_times = Some(4);
	} else {
		press_times = None;
	}

	if press_times.is_none() {
		ui.invoke_set_module_answer(crate::logic::Pages::Button, todo!());
		return;
	}

	let press_times = press_times.unwrap();

	press_times_str = SharedString::from(press_times.to_string());
	if press_times > 2 {
		direction = SharedString::from("Up");
	} else {
		direction = SharedString::from("Down");
	}
}
