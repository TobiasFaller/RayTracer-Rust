use vecmath::Vector3;

use camera::RayTraceCamera;
use color::RayTraceColor;
use hit::RayTraceRayHit;
use params::RayTraceParams;
use ray::RayTraceRay;
use scene::RayTraceScene;

pub trait RayTraceShading {
	fn apply(&self, ray: &RayTraceRay, ray_hit: &RayTraceRayHit, camera: &Box<RayTraceCamera>, scene: &RayTraceScene,
		params: &RayTraceParams) -> (RayTraceColor, RayTraceColor);
}

pub trait RayTraceLight: Sync + Send {
	fn init(&mut self, frame: usize);
	fn get_position(&self) -> Vector3<f64>;
	fn get_light(&self, ray: &RayTraceRay) -> RayTraceColor;
}
