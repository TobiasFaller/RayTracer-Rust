use rand::{Rng, thread_rng};
use std::mem::swap;

use color::RayTraceColor;
use light::RayTraceShading;
use sample::RayTraceSampleFilter;

pub trait RayTraceJitter {
	fn apply(&self, x: f64, y: f64) -> (f64, f64);
	fn get_ray_count(&self) -> usize;
}

#[allow(dead_code)]
pub struct RayTraceOutputParams {
	width: usize,
	height: usize,
	frames: usize
}

#[allow(dead_code)]
impl RayTraceOutputParams {
	pub fn new(width: usize, height: usize, frames: usize) -> Self {
		Self {
			width: width,
			height: height,
			frames: frames
		}
	}

	pub fn get_width(&self) -> usize {
		self.width
	}

	pub fn get_height(&self) -> usize {
		self.height
	}

	pub fn get_frames(&self) -> usize {
		self.frames
	}
}

#[allow(dead_code)]
pub struct RayTraceParams {
	ray_jitter: Option<Box<RayTraceJitter + Sync>>,
	filter: Option<Box<RayTraceSampleFilter + Sync>>,
	shading: Option<Box<RayTraceShading + Sync>>,
	max_depth: usize,
	background_color: RayTraceColor,
	indirect_color: RayTraceColor,
	ambient_light: RayTraceColor
}

#[allow(dead_code)]
impl RayTraceParams {
	pub fn new() -> RayTraceParams {
		RayTraceParams {
			ray_jitter: None,
			filter: None,
			max_depth: 3,
			background_color: RayTraceColor::transparent(),
			indirect_color: RayTraceColor::white(),
			ambient_light: RayTraceColor::white(),
			shading: None
		}
	}

	pub fn set_ray_jitter(&mut self, jitter: Option<Box<RayTraceJitter + Sync>>) {
		self.ray_jitter = jitter;
	}

	pub fn get_jitter(&self) -> &Option<Box<RayTraceJitter + Sync>> {
		&self.ray_jitter
	}

	pub fn get_filter(&self) -> &Option<Box<RayTraceSampleFilter + Sync>> {
		&self.filter
	}

	pub fn unwrap_filter(&mut self) -> Option<Box<RayTraceSampleFilter + Sync>> {
		let mut filter = None;
		swap(&mut self.filter, &mut filter);
		filter
	}

	pub fn set_filter(&mut self, filter: Option<Box<RayTraceSampleFilter + Sync>>) {
		self.filter = filter;
	}

	pub fn set_max_depth(&mut self, max_depth: usize) {
		self.max_depth = max_depth;
	}

	pub fn get_max_depth(&self) -> usize {
		self.max_depth
	}

	pub fn set_background_color(&mut self, color: RayTraceColor) {
		self.background_color = color;
	}

	pub fn get_background_color(&self) -> &RayTraceColor {
		&self.background_color
	}

	pub fn set_indirect_color(&mut self, color: RayTraceColor) {
		self.indirect_color = color;
	}

	pub fn get_indirect_color(&self) -> &RayTraceColor {
		&self.indirect_color
	}

	pub fn set_ambient_light(&mut self, ambient_light: RayTraceColor) {
		self.ambient_light = ambient_light;
	}

	pub fn get_ambient_light(&self) -> &RayTraceColor {
		&self.ambient_light
	}

	pub fn get_shading(&self) -> &Option<Box<RayTraceShading + Sync>> {
		&self.shading
	}

	pub fn set_shading(&mut self, shading: Option<Box<RayTraceShading + Sync>>) {
		self.shading = shading;
	}
}

#[allow(dead_code)]
pub struct RayTraceRandomJitter {
	size: f64,
	ray_count: usize
}

#[allow(dead_code)]
impl RayTraceRandomJitter {
	pub fn new() -> Self {
		Self {
			size: 0.2_f64,
			ray_count: 25_usize
		}
	}

	pub fn new_with(size: f64, ray_count: usize) -> Self {
		Self {
			size: size,
			ray_count: ray_count
		}
	}

	pub fn get_size(&self) -> f64 {
		self.size
	}

	pub fn get_ray_count(&self) -> usize {
		self.ray_count
	}
}

impl RayTraceJitter for RayTraceRandomJitter {
	fn get_ray_count(&self) -> usize {
		self.ray_count
	}
	fn apply(&self, x: f64, y: f64) -> (f64, f64) {
		let mut rng = thread_rng();
		(x + rng.gen_range(-1.0, 1.0) * self.size,
		y + rng.gen_range(-1.0, 1.0) * self.size)
	}
}
