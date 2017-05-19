use vecmath::Vector3;

use RayTraceRay;

#[allow(dead_code)]
pub struct AABB {
	x1: Vector3<f64>,
	x2: Vector3<f64>
}

#[allow(dead_code)]
impl AABB {
	pub fn new(x1: Vector3<f64>, x2: Vector3<f64>) -> Self {
		Self {
			x1: x1,
			x2: x2
		}
	}
	
	pub fn is_hit(&self, ray: &RayTraceRay) -> bool {
		false
	}
}