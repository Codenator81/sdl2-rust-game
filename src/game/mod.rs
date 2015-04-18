use sdl2;
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;
use sdl2::rect::Rect;

use sdl2_ge::graphics::Graphics;
use sdl2_ge::texturemanager as t_manager;

pub struct Game<'engine> {
	display:     	Graphics<'engine>,
	running:	 	bool,
	tm:				t_manager::TextureManager,
	source_rect: 	Option<Rect>,
	dest_rect: 		Option<Rect>,
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
		self.source_rect.as_mut().map(|rect| rect.x = 128 * ((sdl2::sdl::get_ticks() / 100) % 6) as i32);
	}

	fn render(&mut self) {
		let mut drawer = self.display.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 255, 0, 0, 255));
		drawer.clear();
		drawer.copy(&self.tm.texture_map["animate"], self.source_rect, self.dest_rect);
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
	//query for size of texture
	//give coords according query from texture
	let source_rect = Some(Rect::new(0, 0, 128, 82));
	let dest_rect = Some(Rect::new(0, 0, 128, 82));
	Game {
	display: display,
	running: true,
	tm: tm,
	source_rect: source_rect,
	dest_rect: dest_rect,
	}
}