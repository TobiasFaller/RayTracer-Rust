use vecmath::Vector3;
use vecmath::vec3_add;
use vecmath::vec3_scale;
use vecmath::vec3_sub;
use vecmath::vec3_normalized;

#[allow(dead_code)]
pub struct RayTraceRay {
	position: Vector3<f64>,
	direction: Vector3<f64>
}

#[allow(dead_code)]
impl RayTraceRay {
	pub fn new(position: Vector3<f64>, direction: Vector3<f64>) -> RayTraceRay {
		RayTraceRay {
			position: position,
			direction: direction
		}
	}

	pub fn new_to(position: Vector3<f64>, to: Vector3<f64>) -> Self {
		let direction = vec3_sub(to, position);
		Self {
			position: position,
			direction: vec3_normalized(direction)
		}
	}

	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}

	pub fn get_direction(&self) -> &Vector3<f64> {
		&self.direction
	}

	pub fn get_position_on_ray(&self, distance: f64) -> Vector3<f64> {
		vec3_add(self.position, vec3_scale(self.direction, distance))
	}
}
