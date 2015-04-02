extern crate sdl2;

pub mod game;

use game::Game;

fn main() {
	let mut g_game = Game::init("Chapter 1", 640, 580);
	g_game.start();
}