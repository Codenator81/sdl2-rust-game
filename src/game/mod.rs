pub mod player;

use sdl2;
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;

use sdl2_ge::graphics::Graphics;
use sdl2_ge::texturemanager as t_manager;

pub struct Game<'engine> {
	display:     	Graphics<'engine>,
	running:	 	bool,
	tm:				t_manager::TextureManager,
	current_frame:	i32,
}

impl <'g>Game <'g>{
	pub fn start(mut self) {
		while self.running
		{
			//read keyboard event
			//handleEvent return true or false and stop loop
			self.handle_events();
			self.update();
			self.render();
		}
	}

	fn update(&mut self) {
		// Every time we want to move another frame, we simply move the location of
		// the source rectangle
		self.current_frame = ((sdl2::sdl::get_ticks() / 100) % 6) as i32;
	}

	fn render(&mut self) {
		let mut drawer = self.display.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 100, 20, 10, 255));
		drawer.clear();
		self.tm.draw("animate".to_string(), 100,0, 128, 82,
			&mut drawer, false);
		self.tm.draw_frame("animate".to_string(), 0,0, 128, 82,
			1, self.current_frame ,&mut drawer, false);
		drawer.present();
	}
	//for now handle close button or Esc key
	fn handle_events(&mut self){
		let mut	event_pump = self.display.context.event_pump();
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

pub fn new(display: Graphics) -> Game {
	let tm = t_manager::load("assets/animate-alpha.png".to_string(),
			"animate".to_string(),
		&display
		);
	Game {
		display: display,
		running: true,
		tm: tm,
		current_frame: 0,
	}
}