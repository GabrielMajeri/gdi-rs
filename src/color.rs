use std::fmt;

/// The color format used by GDI. Uses 24-bits to store the color.
///
/// The underlying format is `0x00_BB_GG_RR`.
#[derive(Copy, Clone)]
pub struct Color(u32);

impl Color {
	/// Creates a new color from the 8-bit red, green and blue color components.
	pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
		let raw = ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);
		Color(raw)
	}

	/// Returns the red component.
	pub fn red(&self) -> u8 {
		self.0 as u8
	}

	/// Returns the green component.
	pub fn green(&self) -> u8 {
		(self.0 >> 8) as u8
	}

	/// Returns the blue component.
	pub fn blue(&self) -> u8 {
		(self.0 >> 16) as u8
	}
}

impl fmt::Debug for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Color {{ r: {}, g: {}, b: {} }}", self.red(), self.green(), self.blue())
	}
}

impl Into<u32> for Color {
	fn into(self) -> u32 {
		self.0
	}
}
