extern crate sdl2;

use sdl2::pixels::Color;

use game::Game;

pub mod game;

fn main() {
	Game::start();
}