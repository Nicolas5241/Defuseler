mod logic;
mod bomb_modules;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	logic::start_application()
}
