//! Object-oriented wrapper for the Windows GDI API.

#![deny(warnings, missing_docs)]

extern crate winapi;
use winapi::shared::{minwindef, windef};
use winapi::um::{wingdi, winuser};

extern crate windows_utils;

#[cfg(test)]
extern crate winit;

mod device_context;
pub use self::device_context::DeviceContext;

mod object;
pub use self::object::Object;

#[macro_use] mod macros;

mod color;
pub use self::color::Color;

mod rectangle;
pub use self::rectangle::Rectangle;

mod brush;
pub use self::brush::Brush;

mod region;
pub use self::region::Region;

pub mod text;
