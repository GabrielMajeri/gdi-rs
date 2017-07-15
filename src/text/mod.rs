//! Text and font manipulation functions.

use windows_utils::str_to_wide;
use {wingdi, DeviceContext};

mod font;

pub use self::font::Font;

/// Writes a character string at the specified location, using the currently selected font, background color, and text color.
pub fn text_out(ctx: &DeviceContext, position: (i32, i32), text: &str) {
	let text = str_to_wide(text);

	let result = unsafe {
		wingdi::TextOutW(
			ctx.as_raw(),
			position.0, position.1,
			text.as_ptr(),
			text.len() as i32
		)
	};

	assert_ne!(result, 0, "Failed to write text.");
}
