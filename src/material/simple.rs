use color::RayTraceColor;

use hit::RayTraceMaterialHit;

use material::RayTraceMaterial;

pub struct RayTraceSimpleMaterial {
	color: RayTraceColor,
	reflectance: f32
}

impl RayTraceSimpleMaterial {
	pub fn new(color: RayTraceColor) -> Self {
		Self {
			color: color,
			reflectance: 0.0
		}
	}

	pub fn new_with_color(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self {
			color: RayTraceColor::new_with(r, g, b, a),
			reflectance: 0.0
		}
	}

	pub fn new_with(color: RayTraceColor, reflectance: f32) -> Self {
		Self {
			color: color,
			reflectance: reflectance
		}
	}
}

#[allow(unused_variables)]
impl RayTraceMaterial for RayTraceSimpleMaterial {
	fn get_hit(&self, x: f64, y: f64) -> RayTraceMaterialHit {
		RayTraceMaterialHit::new_with(self.color.clone(), self.reflectance, 1.0 - self.reflectance, 1.0, 10000.0)
	}
}