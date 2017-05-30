mod cube;
mod plane;
//mod sphere;

pub use self::cube::RayTraceObjectCube;
pub use self::plane::RayTraceObjectPlane;
//pub use self::sphere::RayTraceObjectSphere;

use AABB;
use RayTraceRay;
use hit::RayTraceRayHit;

pub trait RayTraceObject {
	fn init(&mut self, frame: usize);
	fn get_aabb(&self) -> Option<&AABB>;
	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit>;
}
