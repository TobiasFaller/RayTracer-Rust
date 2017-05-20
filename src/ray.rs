use vecmath::Vector3;

use object::RayTraceMaterial;

#[allow(dead_code)]
pub struct RayTraceRay {
	position: Vector3<f64>,
	direction: Vector3<f64>
}

#[allow(dead_code)]
impl RayTraceRay {
	pub fn new(position: Vector3<f64>, direction: Vector3<f64>) -> RayTraceRay {
		RayTraceRay {
			position: position,
			direction: direction
		}
	}

	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}

	pub fn get_direction(&self) -> &Vector3<f64> {
		&self.direction
	}
}

#[allow(dead_code)]
pub struct RayTraceRayHit {
	position: Vector3<f64>,
	surface_normal: Vector3<f64>,
	surface: RayTraceMaterial
}

#[allow(dead_code)]
impl RayTraceRayHit {
	pub fn new(position: Vector3<f64>, surface_normal: Vector3<f64>, surface: RayTraceMaterial) -> RayTraceRayHit {
		RayTraceRayHit {
			position: position,
			surface_normal: surface_normal,
			surface: surface
		}
	}

	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}

	pub fn get_surface_normal(&self) -> &Vector3<f64> {
		&self.surface_normal
	}

	pub fn get_surface_material(&self) -> &RayTraceMaterial {
		&self.surface
	}
}