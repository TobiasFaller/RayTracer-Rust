mod obj_loader;

pub use self::obj_loader::obj_load;

use std::f64;
use std::mem;

use vecmath::Vector3;
use vecmath::Vector2;
use vecmath::{vec3_add, vec3_mul};
use vecmath::row_mat3_transform;

use aabb::AABB;
use anim::RayTraceAnimation;
use hit::RayTraceRayHit;
use material::RayTraceMaterial;
use object::RayTraceObject;
use ray::RayTraceRay;

use math_util::rotate_xyz;

pub struct RayTraceVertice {
	
}

pub struct RayTraceObjectModel {
	material: Box<RayTraceMaterial>,
	size: Vector3<f64>,
	position: Vector3<f64>,
	rotation: Vector3<f64>,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_rot: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_size: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	vertices: Vec<Vector3<f64>>,
	vertex_normals: Vec<Vector3<f64>>,
	texture_normals: Vec<Vector2<f64>>,
	faces: Vec<[Vector3<usize>; 3]>,
	data: Option<WorkingData>
}

struct WorkingData {
	aabb: AABB,
	vertices: Vec<Vector3<f64>>,
	vertex_normals: Vec<Vector3<f64>>
}

impl RayTraceObjectModel {
	fn transform_data(&self, data: &mut WorkingData) {
		let rot_matrix = rotate_xyz(self.rotation);

		for vert in self.vertices.iter() {
			data.vertices.push(
				vec3_add(row_mat3_transform(rot_matrix, vec3_mul(*vert, self.size)), self.position));
		}

		for norm in self.vertex_normals.iter() {
			data.vertex_normals.push(
				vec3_add(row_mat3_transform(rot_matrix, vec3_mul(*norm, self.size)), self.position));
		}
	}
}

impl RayTraceObject for RayTraceObjectModel {
	fn init(&mut self, frame: usize) {
		if let Some(ref anim_pos) = self.anim_pos {
			self.position = anim_pos.next_frame(frame);
		}
		if let Some(ref anim_rot) = self.anim_rot {
			self.rotation = anim_rot.next_frame(frame);
		}
		if let Some(ref anim_size) = self.anim_size {
			self.size = anim_size.next_frame(frame);
		}

		let mut working_data = None;
		if self.data.is_some() {
			mem::swap(&mut working_data, &mut self.data);
		}

		let mut data = if working_data.is_some() { working_data.unwrap() } else {
			WorkingData {
				aabb: AABB::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
				vertices: Vec::new(),
				vertex_normals: Vec::new()
			}
		};

		// Reset size field of vector
		data.vertices.clear();
		data.vertex_normals.clear();

		self.transform_data(&mut data);
		self.data = Some(data);
	}

	fn get_aabb(&self) -> Option<&AABB> {
		if let Some(ref data) = self.data {
			return Some(&data.aabb);
		} else {
			panic!("Model was not initialized!");
		}
	}

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		if let Some(ref data) = self.data {
			return None;
		} else {
			panic!("Model was not initialized!");
		}
	}
}