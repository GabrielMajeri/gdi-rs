//! Object-oriented wrapper for the Windows GDI API.

#![deny(warnings, missing_docs)]

extern crate winapi;
use winapi::shared::{minwindef, windef};
use winapi::um::{wingdi, winuser};

extern crate windows_utils;

#[cfg(test)]
extern crate winit;

mod device_context;
pub use self::device_context::DeviceContext;

mod object;
pub use self::object::Object;

#[macro_use] mod macros;

mod color;
pub use self::color::Color;

mod rectangle;
pub use self::rectangle::Rectangle;

mod brush;
pub use self::brush::Brush;

mod region;
pub use self::region::Region;

pub mod text;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_window() {
		use std::mem;

		let mut events_loop = winit::EventsLoop::new();

		let window = winit::WindowBuilder::new()
			.with_title("gdi-rs window")
			.with_dimensions(800, 600)
			.build(&events_loop)
			.unwrap();

		let hwnd: windef::HWND = unsafe {
			use winit::os::windows::WindowExt;
			mem::transmute(window.get_hwnd())
		};

		{
			let dc = DeviceContext::new(hwnd);

			let scrn = Region::from_rectangle(Rectangle::new(0, 0, 800, 600));
			let background_brush = Brush::from_color(Color::from_rgb(128, 128, 128));

			scrn.fill(&dc, &background_brush);


			let color_brush = Brush::from_color(Color::from_rgb(50, 120, 220));
			let rgn = Region::from_rectangle(Rectangle::new(50, 20, 400, 300));

			let rgn = rgn + Region::from_rectangle(Rectangle::new(100, 40, 300, 500));

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
}
