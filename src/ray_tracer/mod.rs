extern crate vecmath;
extern crate rand;

mod color;
mod basic;
mod ray;
mod object;

pub mod render;
pub mod sink;
pub mod camera;
pub mod anim;
pub mod math_util;

pub use self::color::*;
pub use self::basic::*;
pub use self::ray::*;
pub use self::object::*;