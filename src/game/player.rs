use sdl2::render::{RenderDrawer};

use sdl2_ge::gameobject;
use sdl2_ge::gameobject::{GameObj, GameObject};
use sdl2_ge::texturemanager::TextureManager;

pub struct Player {
	g_obj:	GameObject
}

	pub fn load(x: i32, y: i32, width: i32, height: i32, texture_id: String, tm: TextureManager) -> Player{
		let game_obj = gameobject::load(x, y, width, height, texture_id, tm);
		Player {
			g_obj: game_obj
		}
	}


impl GameObj for Player{
	fn draw(&self, drawer: &mut RenderDrawer) {
		self.g_obj.draw(drawer);
		println!("draw Player object");
	}

	fn update(&mut self) {
		println!("update Player object");
		self.g_obj.x -= 1;
	}
}