use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, Texture};
use sdl2::mouse;
use sdl2::surface::Surface;

use sdl2_image::{self, LoadTexture};

use std::collections::hash_map::{HashMap, Entry};
use std::path::Path;

pub struct Graphics<'g> {
	pub texture:  	HashMap<String, Texture>,
	pub screen: 	Renderer<'g>,
}

impl<'g> Graphics<'g> {
	pub fn init_renderer(sdl_context: &'g sdl2::Sdl , title: &'static str, width: i32, height: i32) -> Renderer<'g> {
		let window = Window::new(
				sdl_context,
				title,
				WindowPos::PosCentered,
				WindowPos::PosCentered,
				width,
				height,
				OPENGL).unwrap();
		Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED).unwrap()
	}

	pub fn new(renderer: Renderer<'g>) -> Graphics<'g> {
		let graphics = Graphics {
			texture:  HashMap::new(),
			screen: renderer,
		};

		mouse::show_cursor(true);
		return graphics;
	}

	pub fn load_image(&mut self, file_path: String) {
		// Load sprite
		let sprite_path = Path::new(&file_path[..]);
		match self.texture.entry(file_path.clone()) {
			Entry::Vacant(entry) => {
				// using sdl2_image lib. It is now in renderer becouse we init it
				match self.screen.load_texture(&sprite_path) {
					Ok(texture) => { entry.insert(texture); },
					Err(msg) => panic!("sprite could not be rendered: {}", msg)
				}
			},
			_ => {},
		};
	}
}