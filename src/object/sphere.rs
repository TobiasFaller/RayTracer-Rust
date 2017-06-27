use std::f64;

use vecmath::Vector3;
use vecmath::{vec3_add, vec3_sub, vec3_scale, vec3_neg, vec3_dot, vec3_square_len, vec3_normalized_sub};
use vecmath::row_mat3_transform;

use aabb::AABB;
use anim::RayTraceAnimation;
use hit::RayTraceRayHit;
use material::RayTraceMaterial;
use object::RayTraceObject;
use ray::RayTraceRay;

use math_util::compute_plane_hit;
use math_util::rotate_xyz;

#[allow(dead_code)]
pub struct RayTraceObjectSphere {
	material: Box<RayTraceMaterial>,
	size: f64,
	center: Vector3<f64>,
	rotation: Vector3<f64>,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_rot: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_size: Option<Box<RayTraceAnimation<f64>>>,
	data: Option<WorkingData>
}

#[allow(dead_code)]
impl RayTraceObjectSphere {
	pub fn new(center: Vector3<f64>, size: f64, material: Box<RayTraceMaterial>) -> Self {
		Self {
			material: material,
			center: center,
			size: size,
			rotation: [0.0, 0.0, 0.0],
			anim_rot: None,
			anim_pos: None,
			anim_size: None,
			data: None
		}
	}

	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
		self.data = None;
	}

	pub fn set_position(&mut self, position: Vector3<f64>) {
		self.center = position;
		self.data = None;
	}

	pub fn set_size(&mut self, size: f64) {
		self.size = size;
		self.data = None;
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

	pub fn set_anim_size_opt(&mut self, anim: Option<Box<RayTraceAnimation<f64>>>) {
		self.anim_size = anim;
	}

	pub fn set_anim_size(&mut self, anim: Box<RayTraceAnimation<f64>>) {
		self.anim_size = Some(anim);
	}
}

struct WorkingData {
	aabb: AABB
}

#[allow(unused_variables)]
impl RayTraceObject for RayTraceObjectSphere {
	fn init(&mut self, frame: usize) {
		if let Some(ref anim_pos) = self.anim_pos {
			self.center = anim_pos.next_frame(frame);
		}
		if let Some(ref anim_rot) = self.anim_rot {
			self.rotation = anim_rot.next_frame(frame);
		}
		if let Some(ref anim_size) = self.anim_size {
			self.size = anim_size.next_frame(frame);
		}

		let rot = rotate_xyz(self.rotation);

		let size_vec = [self.size, self.size, self.size];
		self.data = Some(WorkingData {
				aabb: AABB::new(vec3_sub(self.center, size_vec), vec3_add(self.center, size_vec))
			});
	}

	fn get_aabb(&self) -> Option<&AABB> {
		if let Some(ref data) = self.data {
			return Some(&data.aabb);
		} else {
			panic!("Qube was not initialized!");
		}
	}

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		if let Some(ref data) = self.data {
			let l = ray.get_direction().clone();
			let o = ray.get_position().clone();

			let dist = vec3_sub(o, self.center);

			let a = vec3_square_len(l);
			let b = vec3_dot(l, dist) * 2.0;
			let c = vec3_square_len(dist) - self.size * self.size / 4.0;

			let disc = b * b - 4.0 * a * c;
			if disc < 0.0 {
				return None;
			}

			let t1 = (-b - disc.sqrt()) / 2.0 / a;
			let t2 = (-b + disc.sqrt()) / 2.0 / a;
			let t = if t1 < 0.0 { if t2 < 0.0 { return None; } else { t2 } } else { t1 };

			let hit_point = ray.get_position_on_ray(t);
			let surface_normal = vec3_normalized_sub(hit_point, self.center);

			return Some(RayTraceRayHit::new(t, hit_point, surface_normal, self.material.get_hit(0.0, 0.0)));
		} else {
			panic!("Qube was not initialized!");
		}
	}
}
