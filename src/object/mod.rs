mod cube;
mod plane;
mod sphere;
pub mod model;

pub use self::cube::RayTraceObjectCube;
pub use self::plane::RayTraceObjectPlane;
pub use self::sphere::RayTraceObjectSphere;
pub use self::model::RayTraceObjectModel;

use aabb::AABB;
use hit::RayTraceRayHit;
use ray::RayTraceRay;

pub trait RayTraceObject: Sync + Send {
	fn init(&mut self, frame: usize);
	fn get_aabb(&self) -> Option<&AABB>;
	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit>;
}
