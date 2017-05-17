use ray_tracer::{RayTraceCamera, RayTraceRay, RayTraceOutputParams};
use ray_tracer::vecmath::*;

#[allow(dead_code)]
pub struct RayTracerCameraPerspective<'a> {
	position: Vector3<f64>,
	rotation: Vector3<f64>,
	width: f64,
	height: f64,
	distance: f64,
	screen: &'a RayTraceOutputParams,
	data: Option<WorkingData>
}

struct WorkingData { }

#[allow(dead_code)]
impl<'a> RayTracerCameraPerspective<'a> {
	pub fn new(screen: &'a RayTraceOutputParams, width: f64, height: f64, distance: f64) -> RayTracerCameraPerspective {
		RayTracerCameraPerspective {
			position: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			width: width,
			height: height,
			distance: distance,
			screen: screen,
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

impl<'a> RayTraceCamera for RayTracerCameraPerspective<'a> {
	fn init(&mut self, frame: usize) {
		// Start with a view into neg z-axis
		let plane_vec1 = [1.0, 0.0, 0.0];
		let plane_vec2 = [0.0, 1.0, 0.0];

		let center = vec3_add(self.position, vec3_scale([0.0, 0.0, -1.0], self.distance));

	}

	fn make_ray(&self, x: f64, y: f64) -> RayTraceRay {
		if let Some(ref data) = self.data {
			// TODO
			RayTraceRay::new(self.position, [0.0, 0.0, 0.0])
		} else {
			panic!("Camera was not initialized!");
		}
	}
}