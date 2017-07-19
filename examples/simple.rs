extern crate winit;
extern crate gdi;
use gdi::*;

use std::mem;

fn create_window() -> (winit::EventsLoop, winit::Window) {
	let events_loop = winit::EventsLoop::new();

	let window = winit::WindowBuilder::new()
		.with_title("gdi-rs window")
		.with_dimensions(800, 600)
		.build(&events_loop)
		.unwrap();

	(events_loop, window)
}

fn main() {
	let (mut events_loop, window) = create_window();

	let hwnd = unsafe {
		use winit::os::windows::WindowExt;
		mem::transmute(window.get_hwnd())
	};

	{
		let dc = DeviceContext::new(hwnd);

		let scrn = Region::from_rectangle(Rectangle::new(0, 0, 800, 600));
		let background_brush = Brush::from_color(Color::from_rgb(0, 50, 120));

		scrn.fill(&dc, &background_brush);

		let color_brush = Brush::from_color(Color::from_rgb(50, 120, 220));
		let rgn = Region::from_rectangle(Rectangle::new(50, 20, 400, 300));

		let rgn = rgn + Region::from_rectangle(Rectangle::new(50, 440, 400, 500))
			+ Region::from_rectangle(Rectangle::new(450, 20, 750, 500));

		rgn.fill(&dc, &color_brush);

		text::text_out(&dc, (100, 100), "Hello WinGDI!");
	}

	events_loop.run_forever(|event| {
		use winit::{Event, WindowEvent, ControlFlow};

		match event {
			Event::WindowEvent { event, .. } => {
				match event {
					WindowEvent::Closed => ControlFlow::Break,
					_ => ControlFlow::Continue
				}
			},
			_ => ControlFlow::Continue
		}
	});
}
