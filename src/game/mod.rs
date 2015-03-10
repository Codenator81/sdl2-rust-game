use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;
use sdl2::event::EventPump;


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
		let mut event_pump = self.sdl_cntx.event_pump();

		while(self.g_running)
		{
			//read keyboard event
			self.handleEvents(event_pump);
//			for event in event_pump.poll_iter() {
//				use sdl2::event::Event;
//
//				match event {
//						Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
//						self.g_running = false
//					},
//						_ => {}
//					}
//			}

			//update();
			self.render();
		}
	}

	fn handleEvents(&mut self, mut event_pump: EventPump) -> Game {
		for event in event_pump.poll_iter()  {
			use sdl2::event::Event;

			self.g_running = match event {
					Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
					false
				},
					_ => {true}
				}
		}
		self
	}

	fn render(&self) {
		let mut drawer = self.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
		drawer.clear();
		drawer.present();
	}
}
