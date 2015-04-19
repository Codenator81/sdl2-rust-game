use sdl2::render::{RenderDrawer};

use graphics::Graphics;
use texturemanager::TextureManager;

pub struct GameObject {
	pub tm: TextureManager,
	pub texture_id: String,

	pub current_frame: i32,
	pub current_row: i32,

	pub x: i32,
	pub y: i32,

	pub width: i32,
	pub height: i32,
}

pub trait GameObj {
	fn draw(&self, gdrawer: &mut RenderDrawer);
	fn update(&mut self);
}

	pub fn load(x: i32, y: i32, width: i32, height: i32, texture_id: String, tm: TextureManager) -> GameObject{
		GameObject {
			tm: tm,
			texture_id: texture_id,
			current_frame: 1,
			current_row: 1,
			x: x,
			y: y,
			width: width,
			height: height,
		}
	}

impl GameObj for GameObject {
	fn draw(&self, drawer: &mut RenderDrawer) {
		self.tm.draw_frame(self.texture_id.clone(), self.x, self.y, self.width, self.height, self.current_row,
		self.current_frame, drawer, false);
		println!("draw game_object");
	}

	fn update(&mut self) {
		self.x += 1;
		println!("update game_object");
	}
}