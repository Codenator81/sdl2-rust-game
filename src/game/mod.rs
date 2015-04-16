use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;
use sdl2::rect::Rect;

use sdl2_ge::graphics::Graphics;



pub struct Game<'engine> {
	context:     &'engine sdl2::Sdl,
	display:     Graphics<'engine>,
	running:	 bool,
	sourceRect: Option<Rect>,
	destRect: Option<Rect>,
}

impl <'g>Game <'g>{
	pub fn new(renderer: Renderer<'g>, context: &'g sdl2::Sdl) -> Game<'g> {
		let mut display  = Graphics::new(renderer);
		display.load_image(format!("assets/rider.bmp"));
		//query for size of texture
		let txt_query = display.texture["assets/rider.bmp"].query();
		//give coords according query from texture
		let sourceRect = Some(Rect::new(50, 50, 50, 50));
		let destRect = Some(Rect::new(100, 100, txt_query.width, txt_query.height));
		Game {
			display: display,
			context: context,
			running: true,
			sourceRect: sourceRect,
			destRect: destRect,
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


	fn render(&mut self) {
		let mut drawer = self.display.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
		drawer.clear();
		drawer.copy(&self.display.texture["assets/rider.bmp"], self.sourceRect, self.destRect);
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

