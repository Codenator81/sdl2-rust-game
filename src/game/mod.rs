use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::sdl::Sdl;

pub struct Game{
	pub screen: Renderer,
	pub g_running: bool,
	pub sdl_cntx: Sdl
}

impl Game {

	pub fn init(title: &'static str, width: i32, height: i32) -> Game {
		let mut render: Game;
		let sdl_init = sdl2::init(sdl2::INIT_VIDEO).unwrap();
		let window = match Window::new(
			title,
			WindowPos::PosCentered,
			WindowPos::PosCentered,
			width,
			height,
			OPENGL) {
				Ok(window) => window,
				Err(err) => panic!("failed to create window: {}", err)
			};
		match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
				Ok(renderer) => render = Game{screen: renderer, g_running: true, sdl_cntx: sdl_init},
				Err(err) => panic!("failed to create renderer: {}", err)
			};
		// start the main loop
		render
	}

	pub fn start(&mut self) {
		println!("initalizing sdl2 ...");
		let mut count = 0;
		while count < 1000
		//while(g_Game.g_running)
		{
			//read keyboard event
			//g_Game.handleEvents();
			//update();
			self.render();
			count += 1; // for stop while loop
		}
	}

	pub fn render(&self) {
		let mut drawer = self.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
		drawer.clear();
		drawer.present();
	}
}
