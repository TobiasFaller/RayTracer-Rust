use vecmath::row_mat3_transform;
use vecmath::vec3_dot;
use vecmath::Vector3;

use aabb::AABB;
use anim::RayTraceAnimation;
use hit::RayTraceRayHit;
use material::RayTraceMaterial;
use object::RayTraceObject;
use object::RayTraceHitable;
use ray::RayTraceRay;

use math_util::compute_plane_hit;
use math_util::rotate_xyz;

#[allow(dead_code)]
pub struct RayTraceObjectPlane {
	center: Vector3<f64>,
	rotation: Vector3<f64>,
	material: Box<RayTraceMaterial>,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_rot: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	data: Option<WorkingData>
}

#[allow(dead_code)]
impl RayTraceObjectPlane {
	pub fn new(center: Vector3<f64>, rotation: Vector3<f64>, material: Box<RayTraceMaterial>) -> Self {
		Self {
			center: center,
			rotation: rotation,
			material: material,
			anim_rot: None,
			anim_pos: None,
			data: None
		}
	}

	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
	}

	pub fn set_position(&mut self, position: Vector3<f64>) {
		self.center = position;
	}

	pub fn set_anim_pos_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_pos = anim;
	}

	pub fn set_anim_pos(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_pos = Some(anim);
	}

	pub fn set_anim_rot_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_rot = anim;
	}

	pub fn set_anim_rot(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_rot = Some(anim);
	}
}

struct WorkingData {
	plane_vec: [Vector3<f64>; 2],
	plane_normal: Vector3<f64>
}

const THRESHOLD: f64 = 1e-10;

#[allow(unused_variables)]
impl RayTraceObject for RayTraceObjectPlane {
	fn init(&mut self, frame: usize) {
		if let Some(ref anim_pos) = self.anim_pos {
			self.center = anim_pos.next_frame(frame);
		}
		if let Some(ref anim_rot) = self.anim_rot {
			self.rotation = anim_rot.next_frame(frame);
		}

		let plane_vec1 = [1.0, 0.0, 0.0];
		let plane_vec2 = [0.0, 1.0, 0.0];
		let plane_vec3 = [0.0, 0.0, 1.0];

		let rot = rotate_xyz(self.rotation);

		let vec1 = row_mat3_transform(rot, plane_vec1);
		let vec2 = row_mat3_transform(rot, plane_vec2);
		let vec3 = row_mat3_transform(rot, plane_vec3);

		self.data = Some(WorkingData {
			plane_vec: [vec1, vec3],
			plane_normal: vec2
		});
	}

	fn get_aabb(&self) -> Option<&AABB> {
		return None;
	}
}

impl RayTraceHitable for RayTraceObjectPlane {
	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		if let Some(ref data) = self.data {
			if vec3_dot(data.plane_normal, ray.get_direction().clone()).abs() <= THRESHOLD {
				return None;
			}

			if let Some((dist, vec1, vec2)) = compute_plane_hit(ray, self.center, data.plane_vec[0],
					data.plane_vec[1]) {
				if dist <= 0.0 {
					return None;
				}

				return Some(RayTraceRayHit::new(dist, ray.get_position_on_ray(dist), data.plane_normal,
						self.material.get_hit(vec1, vec2)));
			} else {
				return None;
			}
		} else {
			panic!("Plane was not initialized!");
		}
	}
}