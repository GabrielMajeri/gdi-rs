/// Represents a rectangular area of space.
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
	/// X coordinate of top-left corner.
	pub left: i32,
	/// Y coordinate of top-left corner.
	pub top: i32,
	/// X coordinate of bottom-right corner.
	pub right: i32,
	/// Y coordinate of bottom-right corner.
	pub bottom: i32
}

impl Rectangle {
	/// Creates a new rectangle from the coordinates of its top-left and bottom-right corners.
	pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
		Rectangle {
			left,
			top,
			right,
			bottom
		}
	}
}
