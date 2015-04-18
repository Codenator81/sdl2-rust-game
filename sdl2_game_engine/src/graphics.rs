use sdl2;
use sdl2::sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, Texture};
use sdl2::mouse;
use sdl2::surface::Surface;
use sdl2::sdl::Sdl;

use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};

use std::collections::hash_map::{HashMap, Entry};
use std::path::Path;

pub struct Graphics<'g> {
	pub context:    sdl2::Sdl,
	pub screen: 	Renderer<'g>,
}

impl<'g> Graphics<'g> {
	pub fn init_renderer(title: &'static str, width: i32, height: i32) -> Graphics<'g> {
		println!("initalizing sdl2 ...");
		let sdl_context = sdl::init(sdl::INIT_EVERYTHING).unwrap();
		sdl2_image::init(INIT_PNG | INIT_JPG);
		println!("initializing rendering context ...");
		let window = Window::new(
				&sdl_context,
				title,
				WindowPos::PosCentered,
				WindowPos::PosCentered,
				width,
				height,
				OPENGL).unwrap();
		let renderer = Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED).unwrap();
		mouse::show_cursor(true);
		Graphics {
			screen: renderer,
			context: sdl_context,
		}
	}
}

pub fn quit() {
	sdl2_image::quit();
}