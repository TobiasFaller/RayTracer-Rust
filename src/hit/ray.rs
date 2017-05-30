use vecmath::Vector3;

use hit::RayTraceMaterialHit;

#[allow(dead_code)]
pub struct RayTraceRayHit {
	distance: f64,
	position: Vector3<f64>,
	surface_normal: Vector3<f64>,
	surface: RayTraceMaterialHit
}

#[allow(dead_code)]
impl RayTraceRayHit {
	pub fn new(distance: f64, position: Vector3<f64>, normal: Vector3<f64>, surface: RayTraceMaterialHit) -> Self {
		Self {
			distance: distance,
			position: position,
			surface_normal: normal,
			surface: surface
		}
	}

	pub fn get_distance(&self) -> f64 {
		self.distance
	}

	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}

	pub fn get_surface_normal(&self) -> &Vector3<f64> {
		&self.surface_normal
	}

	pub fn get_surface_material(&self) -> &RayTraceMaterialHit {
		&self.surface
	}
}