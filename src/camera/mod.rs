mod perspective;
mod orthographic;

pub use self::perspective::RayTracerCameraPerspective;
pub use self::orthographic::RayTracerCameraOrthographic;

use vecmath::Vector3;

use ray::RayTraceRay;

pub trait RayTraceCamera: Send + Sync {
	fn init(&mut self, frame: usize);
	fn make_ray(&self, x: f64, y: f64) -> RayTraceRay;
	fn get_direction(&self) -> Vector3<f64>;
}