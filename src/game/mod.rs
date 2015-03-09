use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};

pub struct Game{
	pub renderer: Renderer,
	pub g_running: bool
}

impl Game {

	pub fn init(title: &'static str, width: i32, height: i32) -> Game {
		println!("initalizing sdl2 ...");
		sdl2::init(sdl2::INIT_VIDEO);
		let render: Game;
		let window = match Window::new(
			title,
			WindowPos::PosCentered,
			WindowPos::PosCentered,
			640,
			480,
			OPENGL) {
				Ok(window) => window,
				Err(err) => panic!("failed to create window: {}", err)
			};
		match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
				Ok(renderer) => render = Game{renderer: renderer, g_running: true},
				Err(err) => panic!("failed to create renderer: {}", err)
			};
		// start the main loop
		render
	}

	pub fn render(&self) {
		let mut drawer = self.renderer.drawer();
		drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
		drawer.clear();
		drawer.present();
	}
}
