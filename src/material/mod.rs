mod simple;

pub use self::simple::RayTraceSimpleMaterial;

use hit::RayTraceMaterialHit;

pub trait RayTraceMaterial {
	fn get_hit(&self, x: f64, y: f64) -> RayTraceMaterialHit;
}