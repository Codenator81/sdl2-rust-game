extern crate sdl2;
use sdl2::video::{Window, WindowPos, SHOWN};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};

fn main() {
	sdl2::init(sdl2::INIT_EVERYTHING);
	// загружаем окно и ловим ошибки
	let window = match Window::new("Chapter 1: Setting up SDL",
		WindowPos::PosCentered,
		WindowPos::PosCentered,
		640,
		480,
		SHOWN) {
			Ok(window) => window,
			Err(err) => panic!("failed to create window: {}", err)
	};
	let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
		Ok(renderer) => renderer,
		Err(err) => panic!("failed to create renderer: {}", err)
	};
	
	let mut drawer = renderer.drawer();
	// Заполняем чёрным светом эта функция принимает значения 
	// RGBA альфа в порядке Red, Green, Blue и Alpha
	drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
	drawer.clear();
	drawer.present();
	
	sdl2::timer::delay(5000);
	sdl2::quit();


}