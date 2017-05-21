#![feature(box_syntax, box_patterns)]
#![feature(slice_patterns)]

#[macro_use]
extern crate log;

extern crate vecmath;
extern crate rand;

mod color;
mod basic;
mod ray;
mod aabb;

pub mod render;
pub mod sink;
pub mod camera;
pub mod anim;
pub mod math_util;
pub mod object;

pub use self::color::RayTraceColor;
pub use self::aabb::AABB;

pub use self::basic::*;
pub use self::ray::*;