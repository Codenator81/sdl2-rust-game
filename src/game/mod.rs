use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;
use sdl2::event::EventPump;

pub struct Game{
	pub screen: Renderer,
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
				Ok(renderer) => render = Game{screen: renderer, sdl_cntx: sdl_init},
				Err(err) => panic!("failed to create renderer: {}", err)
			};
		// start the main loop
		render
	}

	pub fn start(&mut self) {
		println!("initalizing sdl2 ...");
		let mut running = true;
		while running
		{
			//read keyboard event
			//handleEvent return true or false and stop loop
			running = Game::handleEvents(&mut self.sdl_cntx.event_pump());
			//update();
			self.render();
		}
	}


	fn render(&self) {
		let mut drawer = self.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
		drawer.clear();
		drawer.present();
	}

	fn handleEvents(event_pump: &mut EventPump)-> bool {
		for event in event_pump.poll_iter()  {
			use sdl2::event::Event;

			match event {
					Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
					println!("false");
					return false
				},
					_ => {true}
				};
		}
		true
	}
}
