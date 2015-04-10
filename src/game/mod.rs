use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;

use sdl2_ge::graphics::Graphics;



pub struct Game<'engine> {
	context:     &'engine sdl2::Sdl,
	display:     Graphics<'engine>,
	running:	 bool
}

impl <'g>Game <'g>{
	pub fn new(renderer: &'g Renderer, context: &'g sdl2::Sdl) -> Game<'g> {
		let display  = Graphics::new(renderer);
		Game {
			display: display,
			context: context,
			running: true
		}
	}

	pub fn start(mut self) {
		while self.running
		{
			//read keyboard event
			//handleEvent return true or false and stop loop
			self.handle_events();
			//self.update();
			self.render();
		}
	}


	fn render(&self) {
		let mut drawer = self.display.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
		drawer.clear();
		drawer.present();
	}
	//for now handle close button or Esc key
	fn handle_events(&mut self){
		let mut	event_pump = self.context.event_pump();
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
	}
}

