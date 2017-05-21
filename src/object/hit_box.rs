use vecmath::Vector3;

use {RayTraceRay, RayTraceRayHit, AABB};
use object::{RayTraceMaterial, RayTraceObject};

#[allow(dead_code)]
pub struct RayTraceObjectHitBox {
	aabb: AABB,
	material: RayTraceMaterial
}

#[allow(dead_code)]
impl RayTraceObjectHitBox {
	pub fn new(vec1: Vector3<f64>, vec2: Vector3<f64>, material: RayTraceMaterial) -> Self {
		Self {
			aabb: AABB::new(vec1, vec2),
			material: material
		}
	}
}

#[allow(unused_variables)]
impl RayTraceObject for RayTraceObjectHitBox {
	fn init(&mut self, frame: usize) { }

	fn get_aabb(&self) -> Option<&AABB> {
		return Some(&self.aabb);
	}

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		return Some(RayTraceRayHit::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], self.material.clone()));
	}
}