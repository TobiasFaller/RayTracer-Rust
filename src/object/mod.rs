mod hit_box;
//mod plane;
//mod sphere;

pub use self::hit_box::RayTraceObjectHitBox;
//pub use self::plane::RayTraceObjectPlane;
//pub use self::sphere::RayTraceObjectSphere;

use {RayTraceRay, RayTraceRayHit, RayTraceColor, AABB};

pub trait RayTraceObject {
	fn init(&mut self, frame: usize);
	fn get_aabb(&self) -> Option<&AABB>;
	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit>;
}

#[allow(dead_code)]
#[derive(Copy)]
pub struct RayTraceMaterial {
	color: RayTraceColor
}

#[allow(dead_code)]
impl RayTraceMaterial {
	pub fn new(color: RayTraceColor) -> RayTraceMaterial {
		RayTraceMaterial {
			color: color
		}
	}

	pub fn get_color(&self) -> &RayTraceColor {
		&self.color
	}
}

impl Clone for RayTraceMaterial {
	fn clone(&self) -> RayTraceMaterial {
		RayTraceMaterial {
			color: self.color.clone()
		}
	}
}