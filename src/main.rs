extern crate sdl2;

use sdl2::pixels::Color;

use game::Game;

pub mod game;

fn main() {
	let mut g_Game = Game::init("Chapter 1", 640, 480);
	let mut count = 0;
	while count < 1000
	//while(g_Game.g_running)
	{
		//read keyboard event
		//g_Game.handleEvents();
		//update();
		g_Game.render();
		count += 1; // for stop while loop
	}
}