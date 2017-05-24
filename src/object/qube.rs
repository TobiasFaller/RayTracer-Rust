use vecmath::Vector3;
use vecmath::{vec3_add, vec3_sub, vec3_scale, vec3_neg};
use vecmath::{vec4_scale, vec4_sub};
use vecmath::row_mat3_transform;

use {RayTraceRay, RayTraceRayHit, AABB};
use math_util::rotate_xyz;
use object::{RayTraceMaterial, RayTraceObject};

#[allow(dead_code)]
pub struct RayTraceObjectQube {
	aabb: AABB,
	material: RayTraceMaterial,
	size: Vector3<f64>,
	center: Vector3<f64>,
	rotation: Vector3<f64>,
	data: Option<WorkingData>
}

#[allow(dead_code)]
impl RayTraceObjectQube {
	pub fn new(center: Vector3<f64>, size: Vector3<f64>, material: RayTraceMaterial) -> Self {
		Self {
			aabb: get_aabb(center, size),
			material: material,
			center: center,
			size: size,
			rotation: [0.0, 0.0, 0.0],
			data: None
		}
	}

	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
	}

	pub fn set_position(&mut self, position: Vector3<f64>) {
		self.center = position;
	}
}

fn get_aabb(center: Vector3<f64>, size: Vector3<f64>) -> AABB {
	return AABB::new(vec3_sub(center, vec3_scale(size, 0.70711)), vec3_sub(center, vec3_scale(size, 0.70711)));
}

struct WorkingData {
	plane_vec: [Vector3<f64>; 3],
	plane_center: [Vector3<f64>; 6]
}

#[allow(unused_variables)]
impl RayTraceObject for RayTraceObjectQube {
	fn init(&mut self, frame: usize) {
		let plane_vec1 = [1.0, 0.0, 0.0];
		let plane_vec2 = [0.0, 1.0, 0.0];
		let plane_vec3 = [0.0, 0.0, 1.0];

		let rot = rotate_xyz(self.rotation);

		let vec1 = row_mat3_transform(rot, plane_vec1);
		let vec2 = row_mat3_transform(rot, plane_vec2);
		let vec3 = row_mat3_transform(rot, plane_vec3);

		let vec1_scaled = vec3_scale(vec1, 0.5);
		let vec2_scaled = vec3_scale(vec2, 0.5);
		let vec3_scaled = vec3_scale(vec3, 0.5);

		self.data = Some(WorkingData {
			plane_vec: [vec1, vec2, vec3],
			plane_center: [
				vec3_add(self.center, vec1_scaled),
				vec3_sub(self.center, vec1_scaled),
				vec3_add(self.center, vec2_scaled),
				vec3_sub(self.center, vec2_scaled),
				vec3_add(self.center, vec3_scaled),
				vec3_sub(self.center, vec3_scaled),
			]
		});
	}

	fn get_aabb(&self) -> Option<&AABB> {
		return Some(&self.aabb);
	}

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		if let Some(ref data) = self.data {
			if let Some(hit) = get_plane_hit(ray, data.plane_center[0], &self.size,
					data.plane_vec[0], data.plane_vec, 1, 2, &self.material) {
				return Some(hit);
			}
			if let Some(hit) = get_plane_hit(ray, data.plane_center[1], &self.size,
					vec3_neg(data.plane_vec[0]), data.plane_vec, 1, 2, &self.material) {
				return Some(hit);
			}

			if let Some(hit) = get_plane_hit(ray, data.plane_center[2], &self.size,
					data.plane_vec[1], data.plane_vec, 0, 2, &self.material) {
				return Some(hit);
			}
			if let Some(hit) = get_plane_hit(ray, data.plane_center[3], &self.size,
					vec3_neg(data.plane_vec[1]), data.plane_vec, 0, 2, &self.material) {
				return Some(hit);
			}

			if let Some(hit) = get_plane_hit(ray, data.plane_center[4], &self.size,
					data.plane_vec[2], data.plane_vec, 0, 1, &self.material) {
				return Some(hit);
			}
			if let Some(hit) = get_plane_hit(ray, data.plane_center[5], &self.size,
					vec3_neg(data.plane_vec[2]), data.plane_vec, 0, 1, &self.material) {
				return Some(hit);
			}

			return None;
		} else {
			panic!("Qube was not initialized!");
		}
	}
}

const THRESHOLD: f64 = 1e-10;

fn get_plane_hit(ray: &RayTraceRay, center: Vector3<f64>, size: &Vector3<f64>, normal_vec: Vector3<f64>,
		vec: [Vector3<f64>; 3], v1: usize, v2: usize, material: &RayTraceMaterial) -> Option<RayTraceRayHit> {
	let ray_pos = ray.get_position();
	let ray_dir = ray.get_direction();

	// Find the hitpoint using the Gauß-Jordan-algorithm
	let mut mat = [
			[ray_dir[0], -vec[v1][0], -vec[v2][0], center[0] - ray_pos[0]],
			[ray_dir[1], -vec[v1][1], -vec[v2][1], center[1] - ray_pos[1]],
			[ray_dir[2], -vec[v1][2], -vec[v2][2], center[2] - ray_pos[2]]
		];

	if mat[0][0].abs() < THRESHOLD {
		if mat[1][0].abs() < THRESHOLD {
			if mat[2][0].abs() < THRESHOLD {
				return None; // Cannot construct a hitpoint
			} else {
				// Swap row 0 and 2
				let tmp = mat[0];
				mat[0] = mat[2];
				mat[2] = tmp;
			}
		} else {
			if mat[2][0].abs() > THRESHOLD {
				mat[2] = vec4_sub(mat[2], vec4_scale(mat[1], mat[2][0] / mat[1][0]));
			}

			// Swap row 0 and 1
			let tmp = mat[0];
			mat[0] = mat[1];
			mat[1] = tmp;
		}
	} else {
		if mat[1][0].abs() > THRESHOLD {
			mat[1] = vec4_sub(mat[1], vec4_scale(mat[0], mat[1][0] / mat[0][0]));
		}
		if mat[2][0].abs() > THRESHOLD {
			mat[2] = vec4_sub(mat[2], vec4_scale(mat[0], mat[2][0] / mat[0][0]));
		}
	}

	if mat[1][1].abs() < THRESHOLD {
		if mat[2][1].abs() < THRESHOLD {
			return None; // Cannot construct a hitpoint
		} else {
			// Swap row 1 and 2
			let tmp = mat[1];
			mat[1] = mat[2];
			mat[2] = tmp;
		}
	} else {
		if mat[2][1].abs() > THRESHOLD {
			mat[2] = vec4_sub(mat[2], vec4_scale(mat[1], mat[2][1] / mat[1][1]));
		}
	}

	mat[0] = vec4_sub(mat[0], vec4_scale(mat[1], mat[0][1] / mat[1][1]));

	mat[0] = vec4_sub(mat[0], vec4_scale(mat[2], mat[0][2] / mat[2][2]));
	mat[1] = vec4_sub(mat[1], vec4_scale(mat[2], mat[1][2] / mat[2][2]));

	if (mat[1][4] / mat[1][1]).abs() > (size[v1] * 0.5) {
		return None; // Ray missed rectangle in direction 1
	}

	if (mat[2][4] / mat[2][2]).abs() > (size[v2] * 0.5) {
		return None; // Ray missed rectangle in direction 2
	}

	let distance = mat[0][4] / mat[0][0];
	return Some(RayTraceRayHit::new(distance, vec3_add(ray_pos.clone(), vec3_scale(ray_dir.clone(), distance)),
			normal_vec, material.clone()));
}