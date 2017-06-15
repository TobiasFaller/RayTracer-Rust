#![feature(box_syntax, box_patterns)]
#![feature(slice_patterns)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;

extern crate vecmath;
extern crate rand;

extern crate time;
extern crate scoped_threadpool;

mod aabb;
mod hit;
mod ray;
mod nonsync;

mod math_util;

pub mod anim;
pub mod camera;
pub mod color;
pub mod light;
pub mod material;
pub mod object;
pub mod params;
pub mod render;
pub mod sink;
pub mod scene;
pub mod source;
pub mod util;