use ray_tracer::{RayTraceCamera, RayTraceRay};
use ray_tracer::vecmath::Vector3;

pub struct RayTracerCameraPerspective {
	position: Vector3<f64>
}

impl RayTracerCameraPerspective {
	pub fn new() -> RayTracerCameraPerspective {
		RayTracerCameraPerspective {
			position: [0.0, 0.0, 0.0]
		}
	}
}

impl RayTraceCamera for RayTracerCameraPerspective {
	fn make_ray(&self, x: f64, y: f64, frame: usize) -> RayTraceRay {
		RayTraceRay::new(self.position, [0.0, 0.0, 0.0])
	}
}