use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, Texture};
use sdl2::mouse;

use std::collections::hash_map::{HashMap};

pub struct Graphics<'g> {
	texture:  HashMap<String, Texture>,
	pub screen: Renderer<'g>,
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
}


//let texture = Game::load_image(&renderer, "assets/rider.bmp");
//game.load_image("assets/rider.bmp");

//	pub fn init(&'gm mut self) -> Game{
//		let texture = Game::load_image(&self.screen, "assets/rider.bmp");
//		self.texture = Some(texture);
//		*self
//	}
//fn load_image<'a: 'gm>(screen: &'a Renderer, file_path: &'static str) -> Texture<'a>{
//				let sprite_path = Path::new(file_path);
//				let texture = Surface::from_bmp(&sprite_path).and_then(|surface| {
//							screen.create_texture_from_surface(&surface).and_then(|texture| {
//
//				Ok(texture)
//				})
//			}).unwrap();
//		texture
//
////		let sprite_path = Path::new(file_path);
////		println!("{:?}",sprite_path);
////		let tmp_sprite = Surface::from_bmp(&sprite_path).unwrap();
////		renderer.create_texture_from_surface(&tmp_sprite)
////		(texture, renderer)
//	}