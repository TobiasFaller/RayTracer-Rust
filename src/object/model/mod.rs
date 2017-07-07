mod obj_loader;

pub use self::obj_loader::obj_load;

use std::f64;
use std::mem;
use std::collections::BinaryHeap;

use vecmath::Vector3;
use vecmath::Vector2;
use vecmath::{vec3_add, vec3_mul, vec3_sub, vec3_cross, vec3_normalized};
use vecmath::row_mat3_transform;

use aabb::AABB;
use anim::RayTraceAnimation;
use hit::RayTraceHitHeapEntry;
use hit::RayTraceRayHit;
use material::RayTraceMaterial;
use object::RayTraceObject;
use object::RayTraceHitable;
use octree::RayTraceOctree;
use octree::RayTraceOctreeItem;
use ray::RayTraceRay;

use math_util::rotate_xyz;
use math_util::compute_plane_hit;

pub struct RayTraceObjectModel {
	material: Box<RayTraceMaterial>,
	interpolation: RayTraceModelNormalInterpolation,
	scale: Vector3<f64>,
	position: Vector3<f64>,
	rotation: Vector3<f64>,
	offset: Vector3<f64>,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_rot: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_scale: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	vertices: Vec<Vector3<f64>>,
	vertex_normals: Vec<Vector3<f64>>,
	texture_normals: Vec<Vector2<f64>>,
	faces: Vec<[Vector3<usize>; 3]>,
	data: Option<WorkingData>
}

pub enum RayTraceModelNormalInterpolation {
	Average,
	Linear
}

struct WorkingData {
	aabb: Option<AABB>,
	tree: Option<RayTraceOctree>,
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
		let face_normal = vec3_normalized(vec3_cross(self.vec[0], self.vec[1]));

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

impl RayTraceHitable for Face {
	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		// TODO
		None
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

	pub fn set_scale(&mut self, scale: Vector3<f64>) {
		self.scale = scale;
	}

	pub fn set_offset(&mut self, offset: Vector3<f64>) {
		self.offset = offset;
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

	pub fn set_anim_scale_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_scale = anim;
	}

	pub fn set_anim_scale(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_scale = Some(anim);
	}

	fn transform_data(&self, data: &mut WorkingData) {
		// Reset stored data
		data.aabb = None;
		data.faces.clear();
		data.vertex_normals.clear();

		let rot_matrix = rotate_xyz(self.rotation);

		let mut vertices = Vec::with_capacity(self.vertices.len());
		for vert in self.vertices.iter() {
			let vec = vec3_add(row_mat3_transform(rot_matrix, vec3_mul(vec3_sub(*vert, self.offset), self.scale)),
				self.position);
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
			data.vertex_normals.push(vec3_normalized(row_mat3_transform(rot_matrix, *norm)));
		}

		let mut tree = RayTraceOctree::new(data.aabb.as_ref().unwrap().clone());
		for (id, face) in self.faces.iter().enumerate() {
			let v1 = vertices[face[0][0] - 1];
			let v2 = vertices[face[1][0] - 1];
			let v3 = vertices[face[2][0] - 1];

			let pos = v1;
			let vec1 = vec3_sub(v2, v1);
			let vec2 = vec3_sub(v3, v1);
			let face = Face {
					id: id,
					position: pos,
					vec: [vec1, vec2]
				};

			println!("Adding face {}", id);
			let face = box face;

			let hitable: &Box<RayTraceHitable> = &face;
			tree.add(hitable as *const _);
			data.faces.push(face);
		}

		data.tree = Some(tree);
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
		if let Some(ref anim_scale) = self.anim_scale {
			self.scale = anim_scale.next_frame(frame);
		}

		let mut working_data = None;
		if self.data.is_some() {
			mem::swap(&mut working_data, &mut self.data);
		}

		let mut data = if working_data.is_some() { working_data.unwrap() } else {
			WorkingData {
				aabb: None,
				tree: None,
				vertex_normals: Vec::new(),
				faces: Vec::new()
			}
		};

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
}

impl RayTraceHitable for RayTraceObjectModel {
	fn next_hit(&self, ray: &RayTraceRay) -> Option<RayTraceRayHit> {
		if let Some(ref data) = self.data {
			// Collect all ray hits
			let mut ray_hits = BinaryHeap::<RayTraceHitHeapEntry<RayTraceRayHit>>::new();

			for hit in data.tree.as_ref().unwrap().get_hits(ray) {
				let index;

				match hit {
					RayTraceOctreeItem::FlushGroup => {
						if !ray_hits.is_empty() {
							break;
						}

						continue;
					},
					RayTraceOctreeItem::Item(obj_index) => {
						index = obj_index;
					}
				}

				let face = &data.faces[index];
				if let Some((dist, vec1, vec2)) = compute_plane_hit(ray, face.position, face.vec[0], face.vec[1]) {
					if vec1 < 0.0 || vec1 > 1.0 || vec2 < 0.0 || vec2 > 1.0 || vec1 + vec2 > 1.0 {
						continue; // Missed triangle
					}

					let normals = face.get_normals(&self.faces, &data.vertex_normals, &self.texture_normals);

					let surface_normal;
					let texture_normal;
					match self.interpolation {
						RayTraceModelNormalInterpolation::Average => {
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

					ray_hits.push(RayTraceHitHeapEntry::new(dist,
							RayTraceRayHit::new(dist, ray.get_position_on_ray(dist), surface_normal, material_hit)));
				}
			}

			match ray_hits.pop() {
				None => { return None; },
				Some(hit) => {
					return Some(hit.value);
				}
			}
		} else {
			panic!("Model was not initialized!");
		}
	}
}