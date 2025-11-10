mod logic;
mod bomb_modules;
mod types;
mod impls;
mod traits;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	logic::start_application()
}
