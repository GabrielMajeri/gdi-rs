//! Device contexts are GDI objects used to abstract graphics hardware.
//!
//! A given device context could represent a printer, a graphics card,
//! or an in-memory software renderer.
//!
//! Device contexts store state information required to paint.
//!
//! GDI manages a stack of device contexts. Applications can use the `save`
//! method to save the DC on

use windef::{HDC, HWND};
use {winuser, wingdi};

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

	// TODO: support memory DCs.
	// See https://msdn.microsoft.com/en-us/library/dd183489(v=vs.85).aspx

	/// Saves the current state of the DC.
	pub fn save(&self) -> Result<GraphicsState, ()> {
		match unsafe { wingdi::SaveDC(self.0) } {
			0 => Err(()),
			id => Ok(GraphicsState(id))
		}
	}

	/// Restores this DC to a saved graphics state.
	pub fn restore(&self, state: GraphicsState) -> Result<(), ()> {
		match unsafe { wingdi::RestoreDC(self.0, state.0) } {
			0 => Err(()),
			_ => Ok(())
		}
	}

	/// Returns the raw handle.
	pub fn as_raw(&self) -> HDC {
		self.0
	}
}

/// An index into a GDI-owned stack of saved DC states.
///
/// Applications can use the `restore` function to copy the state back into a DC.
pub struct GraphicsState(i32);

impl Drop for DeviceContext {
	fn drop(&mut self) {
		let result = unsafe {
			winuser::ReleaseDC(self.1, self.0)
		};

		debug_assert_ne!(result, 0);
	}
}
