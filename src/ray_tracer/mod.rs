extern crate vecmath;
extern crate rand;

mod color;
mod basic;

pub mod render;
pub mod sink;
pub mod camera;

pub use self::color::*;
pub use self::basic::*;