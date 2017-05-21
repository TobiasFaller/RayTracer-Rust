/*use vecmath::Vector3;
use vecmath::{vec3_cross, vec3_dot, vec3_normalized};

use {RayTraceRay, RayTraceRayHit, AABB};
use object::{RayTraceMaterial, RayTraceObject};

#[allow(dead_code)]
pub struct RayTraceObjectPlane {
	position: Vector3<f64>,
	vec1: Vector3<f64>,
	vec2: Vector3<f64>,
	material: RayTraceMaterial,
	aabb: AABB
}

#[allow(dead_code)]
impl RayTraceObjectPlane {
	pub fn new(position: Vector3<f64>, vec1: Vector3<f64>, vec2: Vector3<f64>, material: RayTraceMaterial) -> Self {
		Self {
			position: position,
			vec1: vec1,
			vec2: vec2,
			material: material,
			aabb: AABB::new([-0.5, 0.0, -1.5], [0.5, 0.5, -0.5])
		}
	}
}

#[allow(unused_variables)]
impl RayTraceObject for RayTraceObjectPlane {
	fn init(&mut self, frame: usize) { }

	fn get_aabb(&self) -> Option<&AABB> {
		return Some(&self.aabb);
	}

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		let surface_normal: Vector3<f64> = vec3_cross(self.vec1, self.vec2);
		if vec3_dot(surface_normal, ray.get_direction().clone()) == 0.0 {
			return None;
		}

		let n = vec3_normalized(surface_normal);
		// TODO: Calculate hit-point
		return Some(RayTraceRayHit::new(self.position, n, self.material.clone()));
	}
}*/