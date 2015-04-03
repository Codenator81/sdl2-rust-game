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

	pub fn init<'g>(title: &'static str, width: i32, height: i32) -> Game<'g> {
		//let mut render: Game;
		let sdl_init = sdl2::init(sdl2::INIT_VIDEO).unwrap();
		let window = Window::new(
			title,
			WindowPos::PosCentered,
			WindowPos::PosCentered,
			width,
			height,
			OPENGL).unwrap();
		let renderer = Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED).unwrap();
		let sprite_path = Path::new("assets/rider.bmp");
		println!("{:?}",sprite_path);
		let tmp_sprite = Surface::from_bmp(&sprite_path).unwrap();
		let texture = renderer.create_texture_from_surface(&tmp_sprite).unwrap();
		//src/game/mod.rs:37:17: 37:25 error: `renderer` does not live long enough
		//src/game/mod.rs:37              let texture = renderer.create_texture_from_surface(&tmp_sprite).unwrap();
		//												^~~~~~~~
		//src/game/mod.rs:23:76: 47:3 note: reference must be valid for the lifetime 'g as defined on the block at 23:75...
		Game{
			screen: renderer,
			sdl_cntx: sdl_init,
			running: true,
			texture: Some(texture),
			sourceRectangle: None,
			destinationRectangle: None,
		}
	}

	pub fn start(&mut self) {
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
}
