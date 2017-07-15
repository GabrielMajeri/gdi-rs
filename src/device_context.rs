use windef::{HDC, HWND};
use {winuser};

use std::ptr;

/// A context used for drawing.
pub struct DeviceContext(HDC, HWND);

impl DeviceContext {
	/// Creates a new device context for a given window.
	///
	/// Pass a null pointer for the window to get the DC for the entire screen.
	pub fn new(window: HWND) -> Self {
		let handle = unsafe {
			winuser::GetDC(window)
		};

		assert_ne!(handle, ptr::null_mut());

		DeviceContext(handle, window)
	}

	/// Returns the raw handle.
	pub fn as_raw(&self) -> HDC {
		self.0
	}
}

impl Drop for DeviceContext {
	fn drop(&mut self) {
		let result = unsafe {
			winuser::ReleaseDC(self.1, self.0)
		};

		debug_assert_ne!(result, 0);
	}
}
