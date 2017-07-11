use vecmath::Vector3;
use vecmath::vec3_dot;

use color::RayTraceColor;

use camera::RayTraceCamera;
use ray::RayTraceRay;
use hit::RayTraceRayHit;
use light::RayTraceShading;
use params::RayTraceParams;
use scene::RayTraceScene;

use math_util::PI;
use math_util::HALF_PI;
use math_util::RAD_TO_DEG;

pub struct RayTraceDebugAxisShading {
	axis: Vector3<f64>,
	scale: f64,
	offset: f64
}

impl RayTraceDebugAxisShading {
	pub fn new(axis: Vector3<f64>) -> Self {
		Self {
			axis: axis,
			scale: 1.0,
			offset: 0.0
		}
	}

	pub fn new_with(axis: Vector3<f64>, scale: f64, offset: f64) -> Self {
		Self {
			axis: axis,
			scale: scale,
			offset: offset
		}
	}
}

#[allow(unused_variables)]
impl RayTraceShading for RayTraceDebugAxisShading {
	fn apply(&self, ray: &RayTraceRay, ray_hit: &RayTraceRayHit, camera: &Box<RayTraceCamera>, scene: &RayTraceScene,
			params: &RayTraceParams) -> (RayTraceColor, RayTraceColor) {
		let position = ray_hit.get_position();
		let mut factor = vec3_dot(*position, self.axis) * self.scale + self.offset;
		while factor < 0.0 {
			factor += 360.0;
		}
		factor %= 360.0;

		return (RayTraceColor::chroma(factor as f32), RayTraceColor::transparent());
	}
}

pub enum RayTraceDebugNormalType {
	XZ,
	Y,
	Both
}

pub struct RayTraceDebugNormalShading {
	normal: RayTraceDebugNormalType
}

impl RayTraceDebugNormalShading {
	pub fn new(normal: RayTraceDebugNormalType) -> Self {
		Self {
			normal: normal
		}
	}
}

#[allow(unused_variables)]
impl RayTraceShading for RayTraceDebugNormalShading {
	fn apply(&self, ray: &RayTraceRay, ray_hit: &RayTraceRayHit, camera: &Box<RayTraceCamera>, scene: &RayTraceScene,
			params: &RayTraceParams) -> (RayTraceColor, RayTraceColor) {
		let normal = ray_hit.get_surface_normal();

		let mut angle_t = if normal[0] != 0.0 {(normal[0] / normal[2]).atan()} else { 0.0 } + /*3.0 * */HALF_PI;
		let angle_p = normal[1].acos();

		if normal[2] > 0.0 {
			angle_t += /*-*/ PI;
		}

		(match self.normal {
			RayTraceDebugNormalType::XZ => {
				RayTraceColor::chroma((angle_t * RAD_TO_DEG) as f32)
			},
			RayTraceDebugNormalType::Y => {
				RayTraceColor::black()
					.mix(&RayTraceColor::white(), (angle_p * RAD_TO_DEG / 360.0) as f32)
			},
			RayTraceDebugNormalType::Both => {
				RayTraceColor::chroma((angle_t * RAD_TO_DEG) as f32)
					.mix(&RayTraceColor::white(), (angle_p * RAD_TO_DEG / 90.0 - 0.25).max(0.0).min(1.0) as f32)
			}
		}, RayTraceColor::transparent())
	}
}
