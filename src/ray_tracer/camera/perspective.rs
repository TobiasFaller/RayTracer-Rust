use ray_tracer::{RayTraceCamera, RayTraceRay};
use ray_tracer::vecmath::*;

pub struct RayTracerCameraPerspective {
	position: Vector3<f64>,
	rotation: Vector3<f64>,
	width: f64,
	height: f64,
	distance: f64,
	screen_width: usize,
	screen_height: usize,
	data: Option<WorkingData>
}

struct WorkingData { }

impl RayTracerCameraPerspective {
	pub fn new(screen_width: usize, screen_height: usize, width: f64, height: f64, distance: f64) -> RayTracerCameraPerspective {
		RayTracerCameraPerspective {
			position: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			width: width,
			height: height,
			distance: distance,
			screen_height: screen_width,
			screen_width: screen_height,
			data: None
		}
	}

	pub fn set_position(&mut self, postion: Vector3<f64>) {
		self.position = postion;
	}

	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
	}
}

impl RayTraceCamera for RayTracerCameraPerspective {
	fn init(&mut self, frame: usize) {
		let plane_vec1 = [0.0, 1.0, 0.0];
		let plane_vec2 = [0.0, 0.0, 1.0];

		let center = vec3_add(self.position, vec3_scale([1.0, 0.0, 0.0], self.distance));

	}

	fn make_ray(&self, x: f64, y: f64) -> RayTraceRay {
		RayTraceRay::new(self.position, [0.0, 0.0, 0.0])
	}
}