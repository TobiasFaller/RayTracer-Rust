#![feature(box_syntax, box_patterns)]
#![feature(slice_patterns)]

#[macro_use]
extern crate log;

extern crate vecmath;
extern crate rand;

extern crate time;
extern crate scoped_threadpool;

mod aabb;
mod color;
mod ray;

mod math_util;

pub mod anim;
pub mod camera;
pub mod hit;
pub mod material;
pub mod object;
pub mod params;
pub mod render;
pub mod sink;
pub mod scene;
pub mod source;

pub use self::aabb::AABB;
pub use self::color::RayTraceColor;
pub use self::ray::RayTraceRay;
