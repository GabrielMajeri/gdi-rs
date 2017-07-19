//! Functions to manage GDI's graphics batch.

use {wingdi};

/// Flush the calling thread's GDI batch.
///
/// For more information, see the official documentation for
/// [`GdiFlush`](https://msdn.microsoft.com/en-us/library/dd144844(v=vs.85).aspx) for more information.
pub fn flush() -> Result<(), ()> {
	match unsafe { wingdi::GdiFlush() } {
		0 => Err(()),
		_ => Ok(())
	}
}

/// Returns the batch limit for the calling thread.
///
/// When the batch limit is reached, GDI will flush the graphics operations.
pub fn get_batch_limit() -> Result<u32, ()> {
	match unsafe { wingdi::GdiGetBatchLimit() } {
		0 => Err(()),
		limit => Ok(limit)
	}
}

/// Sets the batch limit for the calling thread.
///
/// Passing `0` will cause the batch limit to be set to the default value.
/// Passing `1` will cause the batching mechanism to be disabled.
pub fn set_batch_limit(limit: u32) -> Result<(), ()> {
	match unsafe { wingdi::GdiSetBatchLimit(limit) } {
		0 => Err(()),
		_ => Ok(())
	}
}
