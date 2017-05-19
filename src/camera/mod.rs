mod perspective;
mod orthorgonal;

pub use self::perspective::RayTracerCameraPerspective;
pub use self::orthorgonal::RayTracerCameraOrthorgonal;

use RayTraceRay;

pub trait RayTraceCamera {
	fn init(&mut self, frame: usize);
	fn make_ray(&self, x: f64, y: f64) -> RayTraceRay;
}