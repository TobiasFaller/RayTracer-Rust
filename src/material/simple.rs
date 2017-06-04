use color::RayTraceColor;

use hit::RayTraceMaterialHit;

use material::RayTraceMaterial;

pub struct RayTraceSimpleMaterial {
	color: RayTraceColor
}

impl RayTraceSimpleMaterial {
	pub fn new(color: RayTraceColor) -> Self {
		Self {
			color: color
		}
	}

	pub fn new_with_color(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self {
			color: RayTraceColor::new_with(r, g, b, a)
		}
	}
}

#[allow(unused_variables)]
impl RayTraceMaterial for RayTraceSimpleMaterial {
	fn get_hit(&self, x: f64, y: f64) -> RayTraceMaterialHit {
		RayTraceMaterialHit::with_color(self.color)
	}
}