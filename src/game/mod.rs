use sdl2;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, Texture};
use sdl2::sdl::Sdl;
use sdl2::keycode::KeyCode;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use std::path::Path;

pub struct Game<'g> {
	screen: Renderer,
	sdl_cntx: Sdl,
	running: bool,
	texture: Option<Texture<'g>>,
	sourceRectangle: Option<Rect>,
	destinationRectangle: Option<Rect>,
}

impl <'gm>Game <'gm>{

	pub fn init(title: &'static str, width: i32, height: i32) -> Game {
		let mut game: Game;
		let sdl_init = sdl2::init(sdl2::INIT_VIDEO).unwrap();
		let window = Window::new(
			title,
			WindowPos::PosCentered,
			WindowPos::PosCentered,
			width,
			height,
			OPENGL).unwrap();
		let renderer = Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED).unwrap();
		let game = Game{
			screen: renderer,
			sdl_cntx: sdl_init,
			running: true,
			texture: None,
			sourceRectangle: None,
			destinationRectangle: None,
		};
//alex@alex-Aspire-V3-771 ~/Desktop/rust/game/sdl_game/sdl2-rust-game $ cargo run
//Compiling sdltest v0.0.1 (file:///home/alex/Desktop/rust/game/sdl_game/sdl2-rust-game)
//src/game/mod.rs:42:3: 42:7 error: `game` does not live long enough
//src/game/mod.rs:42              game.load_image("assets/rider.bmp");
//^~~~
//note: reference must be valid for the static lifetime...
//src/game/mod.rs:41:4: 44:3 note: ...but borrowed value is only valid for the block suffix following statement 4 at 41:3

		game.load_image("assets/rider.bmp");
		game
	}

	pub fn start(mut self) {
		println!("initalizing sdl2 ...");

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
		let mut drawer = self.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
		drawer.clear();
		drawer.present();
	}
	//for now handle close button or Esc key
	fn handle_events(&mut self){
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
	}

	fn load_image(&'gm mut self, file_path: &'static str) {
		let sprite_path = Path::new(file_path);
		println!("{:?}",sprite_path);
		let tmp_sprite = Surface::from_bmp(&sprite_path).unwrap();
		let texture = self.screen.create_texture_from_surface(&tmp_sprite).unwrap();
		self.texture = Some(texture);
	}
}
