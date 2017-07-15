use minwindef::HRGN;
use {wingdi, Rectangle, DeviceContext, Brush, Object};

use std::{ptr, ops};

/// A region is a rectangle, polygon, or ellipse (or a combination of two or more of these shapes)
/// that can be filled, painted, inverted, framed, and used to perform hit testing (testing for the cursor location).
pub struct Region(HRGN);

impl Region {
	/// Returns a null-sized region, centered around the origin.
	pub fn empty_region() -> Self {
		Self::from_rectangle(Rectangle::new(0, 0, 0, 0))
	}

	/// Creates a rectangular region.
	pub fn from_rectangle(rect: Rectangle) -> Self {
		let handle = unsafe {
			wingdi::CreateRectRgn(
				rect.left,
				rect.top,
				rect.right,
				rect.bottom
			)
		};

		assert_ne!(handle, ptr::null_mut());

		Region(handle)
	}

	/// Fills a region by using the specified brush.
	pub fn fill(&self, ctx: &DeviceContext, brush: &Brush) {
		let result = unsafe {
			wingdi::FillRgn(
				ctx.as_raw(),
				self.0,
				brush.as_raw()
			)
		};

		assert_ne!(result, 0);
	}

	fn combine(&self, other: &Region, operation: i32) -> Self {
		let output = Self::empty_region();
		let result = unsafe {
			wingdi::CombineRgn(
				output.0,
				self.0,
				other.0,
				operation
			)
		};

		assert_ne!(result, wingdi::ERROR);

		output
	}
}

impl ops::Add for Region {
	type Output = Region;
	fn add(self, rhs: Region) -> Region {
		self.combine(&rhs, wingdi::RGN_OR)
	}
}
