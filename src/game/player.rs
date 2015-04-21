use sdl2;
use sdl2::render::{RenderDrawer};

use sdl2_ge::gameobject::{GameObj, GameObject};
use sdl2_ge::texturemanager::TextureManager;

pub struct Player {
	g_obj:	GameObject
}
impl Player {
	pub fn load(x: i32, y: i32, width: i32, height: i32, texture_id: String, tm: TextureManager) -> Player{
		let game_obj = GameObject::load(x, y, width, height, texture_id, tm);
		Player {
			g_obj: game_obj
		}
	}
}

impl GameObj for Player{
	fn draw(&self, drawer: &mut RenderDrawer, flip: bool) {
		self.g_obj.draw(drawer, self.g_obj.direction);
	}

	fn update(&mut self) {
		self.g_obj.current_frame = ((sdl2::sdl::get_ticks() / 100) % 6) as i32;
		if (self.g_obj.direction == true) {
			if (self.g_obj.x > 10) {
				self.g_obj.x -= 3;
			} else {
				self.g_obj.direction = false;
			}
		} else {
			if (self.g_obj.x < 510) {
				self.g_obj.x += 3;
			} else {
				self.g_obj.direction = true;
			}
		}
	}
}