use windef::HBRUSH;
use {Color, wingdi};
use std::ptr;

/// A brush can be used to fill some area when drawing.
pub struct Brush(HBRUSH);

impl Brush {
	/// Creates a new solid brush from a given color.
	pub fn from_color(color: Color) -> Self {
		let handle = unsafe {
			wingdi::CreateSolidBrush(color.into())
		};

		assert_ne!(handle, ptr::null_mut());

		Brush(handle)
	}
}

impl_object!(Brush, HBRUSH);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_brush() {
		let _brush = Brush::from_color(Color::from_rgb(128, 128, 128));
	}
}
