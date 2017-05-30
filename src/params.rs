use rand::{Rng, thread_rng};

use RayTraceColor;

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
	max_depth: usize,
	background_color: RayTraceColor,
	indirect_color: RayTraceColor
}

#[allow(dead_code)]
impl RayTraceParams {
	pub fn new() -> RayTraceParams {
		RayTraceParams {
			ray_jitter: None,
			max_depth: 3,
			background_color: RayTraceColor::transparent(),
			indirect_color: RayTraceColor::white()
		}
	}

	pub fn set_ray_jitter(&mut self, jitter: Option<Box<RayTraceJitter + Sync>>) {
		self.ray_jitter = jitter;
	}
	
	pub fn get_jitter(&self) -> &Option<Box<RayTraceJitter + Sync>> {
		&self.ray_jitter
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
}

pub trait RayTraceJitter {
	fn apply(&self, x: f64, y: f64) -> (f64, f64);
	fn get_ray_count(&self) -> usize;
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