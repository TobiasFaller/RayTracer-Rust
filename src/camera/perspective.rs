use vecmath::*;
use math_util::*;

use {RayTraceRay, RayTraceOutputParams};
use camera::RayTraceCamera;

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

struct WorkingData {
	plane_vec: [Vector3<f64>; 2],
	plane_position: Vector3<f64>
}

#[allow(dead_code)]
impl<'a> RayTracerCameraPerspective<'a> {
	pub fn new(screen: &'a RayTraceOutputParams, width: f64, height: f64, distance: f64) -> Self {
		Self {
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
		self.data = None;
	}

	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
		self.data = None;
	}
}

#[allow(unused_variables)]
impl<'a> RayTraceCamera for RayTracerCameraPerspective<'a> {
	fn init(&mut self, frame: usize) {
		// Start with a view into neg z-axis
		let plane_vec1 = [self.width / (self.screen.get_width() as f64), 0.0, 0.0];
		let plane_vec2 = [0.0, self.height / (self.screen.get_height() as f64), 0.0];
		let normal_vec = [0.0, 0.0, -1.0];

		let rot = rotate_xyz(self.rotation);

		self.data = Some(WorkingData {
			plane_vec: [row_mat3_transform(rot, plane_vec1), row_mat3_transform(rot, plane_vec2)],
			plane_position: vec3_add(self.position, vec3_scale(normal_vec, self.distance))
		});
	}

	fn make_ray(&self, x: f64, y: f64) -> RayTraceRay {
		if let Some(ref data) = self.data {
			let offset_x = vec3_scale(data.plane_vec[0], (x - self.screen.get_width() as f64 / 2.0));
			let offset_y = vec3_scale(data.plane_vec[1], (y - self.screen.get_height() as f64 / 2.0));
			let offset = vec3_add(offset_x, offset_y);

			let head = vec3_add(data.plane_position, offset);
			let direction = vec3_sub(head, self.position);

			return RayTraceRay::new(self.position, vec3_normalized(direction));
		} else {
			panic!("Camera was not initialized!");
		}
	}
}