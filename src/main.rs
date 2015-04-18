extern crate sdl2;
extern crate sdl2_game_engine as sdl2_ge;

pub mod game;

use sdl2_ge::graphics;


use game::Game;

fn main() {
	let renderer = graphics::Graphics::init_renderer("Rust Game", 640, 580);
	println!("Init game");
	let game = Game::new(renderer);
	println!("Start game");
	game.start();
	graphics::quit();
}
