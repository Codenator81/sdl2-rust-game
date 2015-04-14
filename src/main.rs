extern crate sdl2;
extern crate sdl2_game_engine as sdl2_ge;

pub mod game;

use sdl2_ge::graphics;

use sdl2::sdl;

use game::Game;

fn main() {
	println!("initalizing sdl2 ...");
	let sdl_context = sdl::init(sdl::INIT_EVERYTHING).unwrap();
	println!("initializing rendering context ...");
	let renderer = graphics::Graphics::init_renderer(&sdl_context,"Rust Game", 640, 580);
	println!("Init game");
	let game = Game::new(renderer, &sdl_context);
	println!("Start game");
	game.start();
}
