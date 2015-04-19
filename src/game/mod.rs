pub mod player;

use game::player::Player;

use sdl2;
use sdl2::timer;
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;

use sdl2_ge::graphics::Graphics;
use sdl2_ge::texturemanager as t_manager;
use sdl2_ge::gameobject::{GameObject, GameObj};

static DELAY_TIME: u32 = 1000 / 60 as u32;

pub struct Game<'engine> {
	display:     	Graphics<'engine>,
	running:	 	bool,
	//tm:				t_manager::TextureManager,
	//current_frame:	i32,
	go: 			GameObject,
	player:			Player,
}

pub fn new(display: Graphics) -> Game {
	let game_txt = t_manager::load("assets/animate-alpha.png".to_string(),
			"animate".to_string(),
			&display
		);
	let player_txt = t_manager::load("assets/animate-alpha.png".to_string(),
			"animate".to_string(),
		&display
		);
	let go = GameObject::load(100, 100, 128, 82, "animate".to_string(), game_txt);
	let player = Player::load(300, 300, 128, 82, "animate".to_string(), player_txt);
	Game {
		display: display,
		running: true,
		//tm: tm,
		//current_frame: 0,
		go: go,
		player: player,
	}
}

impl <'g>Game <'g>{
	pub fn start(mut self) {
		let mut frame_start: u32;
		let mut frame_time: u32;
		while self.running
		{
			frame_start = timer::get_ticks();
			//read keyboard event
			//handleEvent return true or false and stop loop
			self.handle_events();
			self.update();
			self.render();
			frame_time = timer::get_ticks() - frame_start;
			if frame_time < DELAY_TIME {
					timer::delay((DELAY_TIME - frame_time) as u32);
			}
		}
	}

	fn update(&mut self) {
		// Every time we want to move another frame, we simply move the location of
		// the source rectangle
		//self.current_frame = ((sdl2::sdl::get_ticks() / 100) % 6) as i32;
		self.go.update();
		self.player.update();
	}

	fn render(&mut self) {
		let mut drawer = self.display.screen.drawer();
		drawer.set_draw_color(Color::RGBA( 100, 20, 10, 255));
		drawer.clear();
		self.go.draw(&mut drawer, false);
		self.player.draw(&mut drawer, true);
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