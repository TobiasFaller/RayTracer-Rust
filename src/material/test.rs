use color::RayTraceColor;

use hit::RayTraceMaterialHit;

use material::RayTraceMaterial;

pub struct RayTraceCheckerboardMaterial {
	colors: [RayTraceColor; 2],
	scale: [f64; 2],
	reflectance: f32
}

impl RayTraceCheckerboardMaterial {
	pub fn new() -> Self {
		Self {
			colors: [RayTraceColor::black(), RayTraceColor::white()],
			scale: [1.0, 1.0],
			reflectance: 0.0
		}
	}

	pub fn new_with_colors(colors: [RayTraceColor; 2]) -> Self {
		Self {
			colors: colors,
			scale: [1.0, 1.0],
			reflectance: 0.0
		}
	}

	pub fn new_with(colors: [RayTraceColor; 2], scale: [f64; 2], reflectance: f32) -> Self {
		Self {
			colors: colors,
			scale: scale,
			reflectance: reflectance
		}
	}

	pub fn new_with_size(scale: [f64; 2], reflectance: f32) -> Self {
		Self {
			colors: [RayTraceColor::black(), RayTraceColor::white()],
			scale: scale,
			reflectance: reflectance
		}
	}

	pub fn new_with_reflectance(reflectance: f32) -> Self {
		Self {
			colors: [RayTraceColor::black(), RayTraceColor::white()],
			scale: [1.0, 1.0],
			reflectance: reflectance
		}
	}
}

#[allow(unused_variables)]
impl RayTraceMaterial for RayTraceCheckerboardMaterial {
	fn get_hit(&self, x: f64, y: f64) -> RayTraceMaterialHit {
		let x_scaled = (x / self.scale[0]) as i64 + (if x >= 0.0 {0} else {1});
		let y_scaled = (y / self.scale[1]) as i64 + (if y >= 0.0 {0} else {1});

		RayTraceMaterialHit::new_with(self.colors[(x_scaled + y_scaled) as usize & 0x01].clone(), self.reflectance,
			1.0, 0.5, 100.0)
	}
}