mod simple;
mod test;

pub use self::simple::RayTraceSimpleMaterial;
pub use self::test::RayTraceCheckerboardMaterial;

use hit::RayTraceMaterialHit;

pub trait RayTraceMaterial: Send + Sync {
	fn get_hit(&self, x: f64, y: f64) -> RayTraceMaterialHit;
}