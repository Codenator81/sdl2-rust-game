use sdl2;
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

	pub direction: bool
}

pub trait GameObj {
	fn draw(&self, drawer: &mut RenderDrawer, flip: bool);
	fn update(&mut self);
}
impl GameObject {
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
			direction: true
		}
	}
}

impl GameObj for GameObject {
	fn draw(&self, drawer: &mut RenderDrawer, flip: bool) {
		self.tm.draw_frame(self.texture_id.clone(), self.x, self.y, self.width, self.height, self.current_row,
		self.current_frame, drawer, flip);
	}

	fn update(&mut self) {
		self.current_frame = ((sdl2::sdl::get_ticks() / 100) % 6) as i32;
		self.x += 2;
	}
}