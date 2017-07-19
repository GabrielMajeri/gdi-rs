//! Object-oriented wrapper for the Windows GDI API.
//!
//! This crate is useful for creating static 2D graphics on Windows computers.
//!
//! If you're interested in creating high-performance graphical applications, such
//! as games or CAD software, you should use something like [Piston](http://www.piston.rs/),
//! which uses high-performance graphics APIs, such as DirectX.

#![cfg(windows)]
#![deny(warnings, missing_docs)]

extern crate winapi;
use winapi::shared::{minwindef, windef};
use winapi::um::{wingdi, winuser};

extern crate windows_utils;

pub mod device_context;
pub use self::device_context::DeviceContext;

pub mod batch;

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
