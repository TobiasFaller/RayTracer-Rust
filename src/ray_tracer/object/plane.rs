use ray_tracer::vecmath::Vector3;
use ray_tracer::vecmath::{vec3_cross, vec3_dot, vec3_normalized};

use ray_tracer::{RayTraceObject, RayTraceRay, RayTraceRayHit, RayTraceMaterial};

#[allow(dead_code)]
struct RayTraceObjectPlane {
	position: Vector3<f64>,
	vec1: Vector3<f64>,
	vec2: Vector3<f64>,
	material: RayTraceMaterial
}

#[allow(dead_code)]
impl RayTraceObjectPlane {
	fn new(position: Vector3<f64>, vec1: Vector3<f64>, vec2: Vector3<f64>, material: RayTraceMaterial) -> RayTraceObjectPlane {
		RayTraceObjectPlane {
			position: position,
			vec1: vec1,
			vec2: vec2,
			material: material
		}
	}
}

#[allow(unused_variables)]
impl RayTraceObject for RayTraceObjectPlane {
	fn init(&mut self, frame: usize) { }

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		let surface_normal: Vector3<f64> = vec3_cross(self.vec1, self.vec2);
		if vec3_dot(surface_normal, ray.get_direction().clone()) == 0.0 {
			return None;
		}

		let n = vec3_normalized(surface_normal);
		// TODO: Calculate hit-point
		return Some(RayTraceRayHit::new(self.position, n, self.material.clone()));
	}
}