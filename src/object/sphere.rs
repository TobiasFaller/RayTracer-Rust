use std::f64;

use vecmath::Vector3;
use vecmath::Matrix3;
use vecmath::{vec3_add, vec3_sub, vec3_neg, vec3_dot, vec3_square_len, vec3_normalized_sub};
use vecmath::row_mat3_transform;

use aabb::AABB;
use anim::RayTraceAnimation;
use hit::RayTraceRayHit;
use material::RayTraceMaterial;
use object::RayTraceObject;
use ray::RayTraceRay;

use math_util::PI;
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
	}

	pub fn set_position(&mut self, position: Vector3<f64>) {
		self.center = position;
	}

	pub fn set_size(&mut self, size: f64) {
		self.size = size;
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
	aabb: AABB,
	rot_matrix: Matrix3<f64>
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

		let size_vec = [self.size, self.size, self.size];
		self.data = Some(WorkingData {
				aabb: AABB::new(vec3_sub(self.center, size_vec), vec3_add(self.center, size_vec)),
				rot_matrix: rotate_xyz(vec3_neg(self.rotation))
			});
	}

	fn get_aabb(&self) -> Option<&AABB> {
		if let Some(ref data) = self.data {
			return Some(&data.aabb);
		} else {
			panic!("Sphere was not initialized!");
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

			let tex_normal = row_mat3_transform(data.rot_matrix, surface_normal);
			let angle_t = if tex_normal[0] != 0.0 {(tex_normal[2] / tex_normal[0]).atan()} else { -PI };
			let angle_p = tex_normal[1].acos();

			return Some(RayTraceRayHit::new(t, hit_point, surface_normal, self.material.get_hit(angle_t, angle_p)));
		} else {
			panic!("Sphere was not initialized!");
		}
	}
}
