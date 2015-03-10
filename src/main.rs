extern crate sdl2;

use sdl2::pixels::Color;

use game::Game;

pub mod game;

fn main() {
	let mut g_Game = Game::init("Chapter 1", 640, 480);
	g_Game.start();
}