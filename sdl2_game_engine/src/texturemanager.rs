use sdl2::render::{Renderer, Texture, RenderDrawer};
use sdl2::rect::Rect;

use sdl2_image::{self, LoadTexture};

use std::collections::hash_map::{HashMap, Entry};
use std::path::Path;

use graphics::Graphics;

pub struct TextureManager {
	pub texture_map:  	HashMap<String, Texture>,
}

pub fn load(file_name: String, img_id: String, graph: &Graphics) -> TextureManager{
	// Load sprite
	let mut texture_manager: TextureManager = TextureManager {
		 texture_map:  HashMap::new(),
	};
	let sprite_path = Path::new(&file_name[..]);
	match texture_manager.texture_map.entry(img_id) {
		Entry::Vacant(entry) => {
			// using sdl2_image lib. It is now in renderer becouse we init it
			match graph.screen.load_texture(&sprite_path) {
			Ok(texture) => { entry.insert(texture); },
			Err(msg) => panic!("sprite could not be rendered: {}", msg)
			}
		},
		_ => {},
	};
	texture_manager
}

impl TextureManager {
	pub fn draw(&self, img_id: String, x: i32, y: i32, width: i32, height: i32,
				drawer: &mut RenderDrawer, flip: bool) {
		let mut src_rect: Rect = Rect::new(0, 0, width, height);
		let mut dest_rect: Rect = Rect::new(x, y, width, height);
		let texture = self.texture_map.get(&img_id).unwrap();
		drawer.copy_ex(&texture, Some(src_rect), Some(dest_rect), 0.0, None, (flip, false));
	}
	pub fn draw_frame(&self, img_id: String, x: i32, y: i32, width: i32, height: i32, current_row: i32,
				current_frame: i32, drawer: &mut RenderDrawer, flip: bool) {
		let mut src_rect: Rect = Rect::new(width * current_frame, height * (current_row - 1), width, height);
		let mut dest_rect: Rect = Rect::new(x, y, width, height);
		let texture = self.texture_map.get(&img_id).unwrap();
		drawer.copy_ex(&texture, Some(src_rect), Some(dest_rect), 0.0, None, (flip, false));
	}
}