extern crate sdl2;
use sdl2::video::{Window, WindowPos, SHOWN};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};

pub struct WindowG {
	window: Window
}
pub struct Render {
renderer: Renderer
}

fn main() {
	let mut g_running: bool = true;
	let graph = init("Try SDL");
	let mut count = 0;
	while count < 10000
	//while(g_running) 
	{
		render(&graph);
		count += 1; // for stop while loop
	}	
	sdl2::quit();
}

fn init(title: &'static str) -> Render {
		sdl2::init(sdl2::INIT_EVERYTHING);
		// загружаем окно и ловим ошибки
		let window_g: WindowG;
		let render: Render;
		match Window::new(title,
			WindowPos::PosCentered,
			WindowPos::PosCentered,
			640,
			480,
			SHOWN) {
				Ok(window) => window_g = WindowG {window: window },
				Err(err) => panic!("failed to create window: {}", err)
			};
		match Renderer::from_window(window_g.window, RenderDriverIndex::Auto, ACCELERATED) {
				Ok(renderer) => render = Render{renderer:renderer},
				Err(err) => panic!("failed to create renderer: {}", err)
		};
		render
}
fn render(graph: &Render ) {
	let mut drawer = graph.renderer.drawer();
	// Заполняем чёрным светом эта функция принимает значения
	// RGBA альфа в порядке Red, Green, Blue и Alpha
	drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
	drawer.clear();
	drawer.present();
}