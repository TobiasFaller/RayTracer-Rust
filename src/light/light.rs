use vecmath::Vector3;

use camera::RayTraceCamera;
use color::RayTraceColor;
use hit::RayTraceRayHit;
use params::RayTraceParams;
use ray::RayTraceRay;
use scene::RayTraceScene;

pub trait RayTraceShading {
	fn apply(&self, ray: &RayTraceRay, ray_hit: &RayTraceRayHit, camera: &Box<RayTraceCamera>, scene: &RayTraceScene,
		params: &RayTraceParams) -> RayTraceColor;
}

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
