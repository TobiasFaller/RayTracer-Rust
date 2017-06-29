mod obj_loader;

pub use self::obj_loader::obj_load;

use std::f64;
use std::mem;
use std::cmp::Ordering;

use vecmath::Vector3;
use vecmath::Vector2;
use vecmath::{vec3_add, vec3_mul, vec3_sub, vec3_cross};
use vecmath::row_mat3_transform;

use aabb::AABB;
use anim::RayTraceAnimation;
use hit::RayTraceRayHit;
use material::RayTraceMaterial;
use object::RayTraceObject;
use ray::RayTraceRay;

use math_util::rotate_xyz;
use math_util::compute_plane_hit;

pub struct RayTraceObjectModel {
	material: Box<RayTraceMaterial>,
	shading: RayTraceModelShading,
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

pub enum RayTraceModelShading {
	Flat,
	Soft
}

struct WorkingData {
	aabb: Option<AABB>,
	vertex_normals: Vec<Vector3<f64>>,
	faces: Vec<Face>
}

struct Face {
	id: usize,
	position: Vector3<f64>,
	vec: [Vector3<f64>; 2]
}

impl Face {
	fn get_normals(&self, faces: &Vec<[Vector3<usize>; 3]>, normals: &Vec<Vector3<f64>>,
			texture_normals: &Vec<Vector2<f64>>) -> [(Vector3<f64>, Vector2<f64>); 3] {
		let face = faces[self.id];

		let n = [face[0][1], face[1][1], face[2][1]];
		let t = [face[0][2], face[0][2], face[0][2]];
		let face_normal = vec3_cross(self.vec[0], self.vec[1]);

		[
			(
				if n[0] == 0 { face_normal } else { normals[n[0] - 1] },
				if t[0] == 0 { [0.0, 0.0] } else { texture_normals[t[0] - 1] }
			),
			(
				if n[1] == 0 { face_normal } else { normals[n[1] - 1] },
				if t[1] == 0 { [0.0, 0.0] } else { texture_normals[t[1] - 1] }
			),
			(
				if n[2] == 0 { face_normal } else { normals[n[2] - 1] },
				if t[2] == 0 { [0.0, 0.0] } else { texture_normals[t[2] - 1] }
			)
		]
	}
}

const AABB_MIN_DIST: Vector3<f64> = [0.001, 0.001, 0.001];

impl RayTraceObjectModel {
	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
	}

	pub fn set_position(&mut self, position: Vector3<f64>) {
		self.position = position;
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

	fn transform_data(&self, data: &mut WorkingData) {
		let rot_matrix = rotate_xyz(self.rotation);

		let mut vertices = Vec::with_capacity(self.vertices.len());
		for vert in self.vertices.iter() {
			let vec = vec3_add(row_mat3_transform(rot_matrix, vec3_mul(*vert, self.size)), self.position);
			vertices.push(vec);

			match data.aabb {
				Some(ref mut aabb) => {
					aabb.expand(vec);
				},
				None => {
					data.aabb = Some(AABB::new(vec3_sub(vec, AABB_MIN_DIST), vec3_add(vec, AABB_MIN_DIST)));
				}
			}
		}

		for norm in self.vertex_normals.iter() {
			data.vertex_normals.push(vec3_add(row_mat3_transform(rot_matrix, *norm), self.position));
		}

		for (id, face) in self.faces.iter().enumerate() {
			let v1 = vertices[face[0][0] - 1];
			let v2 = vertices[face[1][0] - 1];
			let v3 = vertices[face[2][0] - 1];

			let pos = v1;
			let vec1 = vec3_sub(v2, v1);
			let vec2 = vec3_sub(v3, v1);

			data.faces.push(Face {
					id: id,
					position: pos,
					vec: [vec1, vec2]
				});
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
				aabb: None,
				vertex_normals: Vec::new(),
				faces: Vec::new()
			}
		};

		// Reset size field of vector
		data.faces.clear();
		data.aabb = None;

		self.transform_data(&mut data);
		self.data = Some(data);
	}

	fn get_aabb(&self) -> Option<&AABB> {
		if let Some(ref data) = self.data {
			return data.aabb.as_ref();
		} else {
			panic!("Model was not initialized!");
		}
	}

	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		if let Some(ref data) = self.data {
			let mut ray_hits: Vec<RayTraceRayHit> = Vec::new();

			for face in data.faces.iter() {
				if let Some((dist, vec1, vec2)) = compute_plane_hit(ray, face.position, face.vec[0], face.vec[1]) {
					if vec1 < 0.0 || vec1 > 1.0 || vec2 < 0.0 || vec2 > 1.0 || vec1 + vec2 > 1.0 {
						continue; // Missed triangle
					}

					let normals = face.get_normals(&self.faces, &data.vertex_normals, &self.texture_normals);

					let surface_normal;
					let texture_normal;
					match self.shading {
						RayTraceModelShading::Flat => {
							surface_normal = [
								(normals[0].0[0] + normals[1].0[0] + normals[2].0[0]) / 3.0,
								(normals[0].0[1] + normals[1].0[1] + normals[2].0[1]) / 3.0,
								(normals[0].0[2] + normals[1].0[2] + normals[2].0[2]) / 3.0
							];
							texture_normal = [
								(normals[0].1[0] + normals[1].1[0] + normals[2].1[0]) / 3.0,
								(normals[0].1[1] + normals[1].1[1] + normals[2].1[1]) / 3.0
							];
						},
						_ => {
							panic!("Unsupported shading model");
						}
					}

					let material_hit = self.material.get_hit(texture_normal[0], texture_normal[1]);

					ray_hits.push(
						RayTraceRayHit::new(dist, ray.get_position_on_ray(dist), surface_normal, material_hit));
				}
			}

			if ray_hits.is_empty() {
				return None;
			}

			ray_hits.sort_by(|a, b| {
				match a.get_distance().partial_cmp(&b.get_distance()) {
					Some(ordering) => {
						ordering
					},
					None => {
						Ordering::Equal
					}
				}
			});

			return Some(ray_hits.remove(0));
		} else {
			panic!("Model was not initialized!");
		}
	}
}