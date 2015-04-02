use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;
use sdl2::event::EventPump;

pub struct Game{
	screen: Renderer,
	sdl_cntx: Sdl,
	running: bool
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
				Ok(renderer) => render = Game{screen: renderer, sdl_cntx: sdl_init, running: true},
				Err(err) => panic!("failed to create renderer: {}", err)
			};
		// start the main loop
		render
	}

	pub fn start(&mut self) {
		println!("initalizing sdl2 ...");
		while self.running
		{
			//read keyboard event
			//handleEvent return true or false and stop loop
			self.handleEvents();
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
	//for now handle close button or Esc key
	fn handleEvents(&mut self){
		let mut	event_pump = self.sdl_cntx.event_pump();
		for event in event_pump.poll_iter()  {
			use sdl2::event::Event;

			match event {
					Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
					println!("Exit game", );
					self.running = false
				},
					_ => {self.running = true}
				};
		}
		self.running = true
	}
}
