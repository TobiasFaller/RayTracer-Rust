use std::f64;

use vecmath::Vector3;
use vecmath::{vec3_add, vec3_sub, vec3_scale, vec3_neg};
use vecmath::row_mat3_transform;

use aabb::AABB;
use anim::RayTraceAnimation;
use hit::RayTraceRayHit;
use material::RayTraceMaterial;
use object::RayTraceObject;
use object::RayTraceHitable;
use ray::RayTraceRay;

use math_util::compute_plane_hit;
use math_util::rotate_xyz;

enum CubeMaterial {
	OnePerCube(Box<RayTraceMaterial>),
	OnePerSide([Box<RayTraceMaterial>; 6])
}

#[allow(dead_code)]
pub struct RayTraceObjectCube {
	material: Box<CubeMaterial>,
	size: Vector3<f64>,
	center: Vector3<f64>,
	rotation: Vector3<f64>,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_rot: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_size: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	data: Option<WorkingData>
}

#[allow(dead_code)]
impl RayTraceObjectCube {
	pub fn new(center: Vector3<f64>, size: Vector3<f64>, material: Box<RayTraceMaterial>) -> Self {
		Self {
			material: box CubeMaterial::OnePerCube(material),
			center: center,
			size: size,
			rotation: [0.0, 0.0, 0.0],
			anim_rot: None,
			anim_pos: None,
			anim_size: None,
			data: None
		}
	}

	pub fn new_with(center: Vector3<f64>, size: Vector3<f64>, materials: [Box<RayTraceMaterial>; 6]) -> Self {
		Self {
			material: box CubeMaterial::OnePerSide(materials),
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

	pub fn set_size(&mut self, size: Vector3<f64>) {
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

	pub fn set_anim_size_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_size = anim;
	}

	pub fn set_anim_size(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_size = Some(anim);
	}

	fn get_material(&self, index: usize) -> &Box<RayTraceMaterial> {
		match self.material {
			box CubeMaterial::OnePerSide(ref materials) => {
				&materials[index]
			},
			box CubeMaterial::OnePerCube(ref material) => {
				&material
			}
		}
	}
}

fn gen_aabb(center: Vector3<f64>, size: Vector3<f64>) -> AABB {
	let dir = vec3_scale(size, 1.42 * 0.5);
	return AABB::new(vec3_sub(center, dir), vec3_add(center, dir));
}

struct WorkingData {
	plane_vec: [Vector3<f64>; 3],
	plane_center: [Vector3<f64>; 6],
	aabb: AABB
}

#[allow(unused_variables)]
impl RayTraceObject for RayTraceObjectCube {
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

		let plane_vec1 = [1.0, 0.0, 0.0];
		let plane_vec2 = [0.0, 1.0, 0.0];
		let plane_vec3 = [0.0, 0.0, 1.0];

		let rot = rotate_xyz(self.rotation);

		let vec1 = row_mat3_transform(rot, plane_vec1);
		let vec2 = row_mat3_transform(rot, plane_vec2);
		let vec3 = row_mat3_transform(rot, plane_vec3);

		let vec1_scaled = vec3_scale(vec1, 0.5 * self.size[0]);
		let vec2_scaled = vec3_scale(vec2, 0.5 * self.size[1]);
		let vec3_scaled = vec3_scale(vec3, 0.5 * self.size[2]);

		self.data = Some(WorkingData {
			plane_vec: [vec1, vec2, vec3],
			plane_center: [
				vec3_add(self.center, vec1_scaled),
				vec3_sub(self.center, vec1_scaled),
				vec3_add(self.center, vec2_scaled),
				vec3_sub(self.center, vec2_scaled),
				vec3_add(self.center, vec3_scaled),
				vec3_sub(self.center, vec3_scaled),
			],
			aabb: gen_aabb(self.center, self.size)
		});
	}

	fn get_aabb(&self) -> Option<&AABB> {
		if let Some(ref data) = self.data {
			return Some(&data.aabb);
		} else {
			panic!("Qube was not initialized!");
		}
	}
}

impl RayTraceHitable for RayTraceObjectCube {
	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		if let Some(ref data) = self.data {
			let mut hit_distance = f64::MAX;
			let mut hit_ret = None;

			if let Some(hit) = get_plane_hit(ray, data.plane_center[0], &self.size,
					data.plane_vec[0], data.plane_vec, 1, 2, self.get_material(0)) {
				hit_distance = hit.get_distance();
				hit_ret = Some(hit);
			}
			if let Some(hit) = get_plane_hit(ray, data.plane_center[1], &self.size,
					vec3_neg(data.plane_vec[0]), data.plane_vec, 1, 2, self.get_material(1)) {
				if hit.get_distance() < hit_distance {
					hit_distance = hit.get_distance();
					hit_ret = Some(hit);
				}
			}

			if let Some(hit) = get_plane_hit(ray, data.plane_center[2], &self.size,
					data.plane_vec[1], data.plane_vec, 0, 2, self.get_material(2)) {
				if hit.get_distance() < hit_distance {
					hit_distance = hit.get_distance();
					hit_ret = Some(hit);
				}
			}
			if let Some(hit) = get_plane_hit(ray, data.plane_center[3], &self.size,
					vec3_neg(data.plane_vec[1]), data.plane_vec, 0, 2, self.get_material(3)) {
				if hit.get_distance() < hit_distance {
					hit_distance = hit.get_distance();
					hit_ret = Some(hit);
				}
			}

			if let Some(hit) = get_plane_hit(ray, data.plane_center[4], &self.size,
					data.plane_vec[2], data.plane_vec, 0, 1, self.get_material(4)) {
				if hit.get_distance() < hit_distance {
					hit_distance = hit.get_distance();
					hit_ret = Some(hit);
				}
			}
			if let Some(hit) = get_plane_hit(ray, data.plane_center[5], &self.size,
					vec3_neg(data.plane_vec[2]), data.plane_vec, 0, 1, self.get_material(5)) {
				if hit.get_distance() < hit_distance {
					hit_ret = Some(hit);
				}
			}

			return hit_ret;
		} else {
			panic!("Qube was not initialized!");
		}
	}
}

fn get_plane_hit(ray: &RayTraceRay, center: Vector3<f64>, size: &Vector3<f64>, normal_vec: Vector3<f64>,
		vec: [Vector3<f64>; 3], v1: usize, v2: usize, material: &Box<RayTraceMaterial>) -> Option<RayTraceRayHit> {
	if let Some((dist, vec1, vec2)) = compute_plane_hit(ray, center, vec[v1], vec[v2]) {
		if dist <= 0.0 {
			return None;
		}

		if vec1.abs() > size[v1] * 0.5 {
			return None;
		}

		if vec2.abs() > size[v2] * 0.5 {
			return None;
		}

		return Some(RayTraceRayHit::new(dist, ray.get_position_on_ray(dist),
				normal_vec, material.get_hit(vec1, vec2)));
	} else {
		return None;
	}
}