#![feature(box_syntax, box_patterns)]
#![feature(slice_patterns)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;
extern crate rand;

extern crate time;
extern crate scoped_threadpool;

pub extern crate vecmath;

mod aabb;
mod hit;
mod ray;
mod nonsync;

pub mod anim;
pub mod camera;
pub mod color;
pub mod light;
pub mod material;
pub mod math_util;
pub mod object;
pub mod params;
pub mod render;
pub mod sample;
pub mod sink;
pub mod scene;
pub mod source;