use vecmath::Vector3;

use RayTraceColor;

pub struct RayTraceSpotLight {
	position: Vector3<f64>,
	color: RayTraceColor
}

impl RayTraceSpotLight {
	pub fn new(position: Vector3<f64>, color: RayTraceColor) -> Self {
		Self {
			position: position,
			color: color
		}
	}

	pub fn get_color(&self) -> &RayTraceColor {
		&self.color
	}

	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}
}
