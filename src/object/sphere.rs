use vecmath::Vector3;

use {RayTraceRay, RayTraceRayHit};
use anim::RayTraceAnimPosition;

struct RayTraceObjectSphere {
	position: Vector3<f64>,
	size: Vector3<f64>,
	anim_position: Option<Box<RayTraceAnimPosition>>,
	final_position: Vector3<f64>
}

impl RayTraceObjectSphere {
	fn new(position: Vector3<f64>, size: Vector3<f64>) -> RayTraceObjectSphere {
		RayTraceObjectSphere {
			position: position,
			size: size,
			anim_position: None,
			final_position: position
		}
	}

	fn set_anim_position(&mut self, animation: Option<Box<RayTraceAnimPosition>>) {
		self.anim_position = animation;
	}
}

/*impl RayTraceObject for RayTraceObjectSphere {
	fn init(&mut self, frame: usize) {
		self.final_position = if let Some(anim) = self.anim_position { anim.new_position(self.position, frame) } else { self.position };
	}

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> { }
}*/