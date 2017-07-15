/// Trait implemented by all GDI objects in the library.
pub trait Object<T> {
	/// Returns the owned handle, without releasing ownership.
	fn as_raw(&self) -> T;

	/// Turns the object into the owned handle.
	///
	/// Note: misusing this function can result in handle leaks.
	fn into_raw(self) -> T;
}
