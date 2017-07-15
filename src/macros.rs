/// Use this macro to implement the `Object` trait.
/// It assumes a tuple struct, where the first element is the handle.
macro_rules! impl_object {
	($name:ident, $handle:ident) => (
		impl ::Object<$handle> for $name {
			fn as_raw(&self) -> $handle {
				self.0
			}

			fn into_raw(self) -> $handle {
				let handle = self.0;
				::std::mem::forget(self);
				handle
			}
		}

		impl Drop for $name {
			fn drop(&mut self) {
				let result = unsafe {
					::wingdi::DeleteObject(self.0 as ::windef::HGDIOBJ)
				};

				debug_assert_ne!(result, 0, "Failed to delete GDI object.");
			}
		}
	)
}
