extern crate sdl2;
use sdl2::video::{Window, WindowPos, SHOWN};
use sdl2::pixels::Color;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};

pub struct Graph {
	renderer: Renderer
}

fn main() {
	sdl2::init(sdl2::INIT_VIDEO);
	let mut g_running: bool = true;
	let graph = init("Try SDL");
	let mut count = 0;
	while count < 1000
	//while(g_running) 
	{
		render(&graph);
		count += 1; // for stop while loop
	}
}

fn init(title: &'static str) -> Graph {
				
		let render: Graph;
		let window = match Window::new(title,
			WindowPos::PosCentered,
			WindowPos::PosCentered,
			640,
			480,
			SHOWN) {
				Ok(window) => window,
				Err(err) => panic!("failed to create window: {}", err)
			};
		match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
				Ok(renderer) => render = Graph{renderer:renderer},
				Err(err) => panic!("failed to create renderer: {}", err)
		};
		render
}

fn render(graph: &Graph ) {
	let mut drawer = graph.renderer.drawer();
	drawer.set_draw_color(Color::RGBA( 0, 0, 0, 255));
	drawer.clear();
	drawer.present();
}