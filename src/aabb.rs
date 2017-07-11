use std::f64;

use vecmath::Vector3;
use vecmath::{vec3_add, vec3_cross, vec3_sub, vec3_dot, vec3_scale, vec3_neg};

use ray::RayTraceRay;

#[allow(dead_code)]
pub struct AABB {
	start: Vector3<f64>,
	end: Vector3<f64>
}

#[allow(dead_code)]
impl AABB {
	pub fn new(x1: Vector3<f64>, x2: Vector3<f64>) -> Self {
		Self {
			start: [x1[0].min(x2[0]), x1[1].min(x2[1]), x1[2].min(x2[2])],
			end: [x1[0].max(x2[0]), x1[1].max(x2[1]), x1[2].max(x2[2])],
		}
	}

	pub fn expand(&mut self, x: Vector3<f64>) {
		self.start[0] = self.start[0].min(x[0]);
		self.start[1] = self.start[1].min(x[1]);
		self.start[2] = self.start[2].min(x[2]);
		self.end[0] = self.end[0].max(x[0]);
		self.end[1] = self.end[1].max(x[1]);
		self.end[2] = self.end[2].max(x[2]);
	}

	pub fn is_intersecting(&self, other: &Self) -> bool {
		for i in 0..2 {
			if self.end[i] < other.start[i] || self.start[i] > other.end[i] {
				return false;
			}
		}

		true
	}

	pub fn is_hit(&self, ray: &RayTraceRay) -> bool {
		let [x, y, z] = *ray.get_position();
		let (x_start, x_end, y_start, y_end, z_start, z_end) = project_points_onto_ray(ray, (self.start, self.end));
		let (mut check_xy, mut check_xz, mut check_yz) = (true, true, true);

		if x_start.is_nan() {
			if x < self.start[0] || x > self.end[0] {
				return false;
			}

			check_xy = false;
			check_xz = false;
		} else if x_start < 0.0 && x_end < 0.0 {
			return false;
		}

		if y_start.is_nan() {
			if y < self.start[1] || y > self.end[1] {
				return false;
			}

			check_xy = false;
			check_yz = false;
		} else if y_start < 0.0 && y_end < 0.0 {
			return false;
		}

		if z_start.is_nan() {
			if z < self.start[2] || z > self.end[2] {
				return false;
			}

			check_xz = false;
			check_yz = false;
		} else if z_start < 0.0 && z_end < 0.0 {
			return false;
		}

		// Check y on x no overlap
		if check_xy && ((y_start < x_start && y_end < x_start) || (y_start > x_end && y_end > x_end)) {
			return false;
		}

		// Check z on x no overlap
		if check_xz && ((z_start < x_start && z_end < x_start) || (z_start > x_end && z_end > x_end)) {
			return false;
		}

		// Check z on y no overlap
		if check_yz && ((z_start < y_start && z_end < y_start) || (z_start > y_end && z_end > y_end)) {
			return false;
		}

		return true;
	}


	pub fn get_first_hit(&self, ray: &RayTraceRay) -> Option<f64> {
		let [x, y, z] = *ray.get_position();
		let (x_start, x_end, y_start, y_end, z_start, z_end) = project_points_onto_ray(ray, (self.start, self.end));
		let (mut check_xy, mut check_xz, mut check_yz) = (true, true, true);
		let mut ray_min = f64::MAX;

		if x_start.is_nan() {
			if x < self.start[0] || x > self.end[0] {
				return None;
			}

			check_xy = false;
			check_xz = false;
		} else if x_start <= 0.0 && x_end <= 0.0 {
			return None;
		} else {
			if x_start > 0.0 {
				ray_min = x_start;
			} else {
				ray_min = x_end;
			}
		}

		if y_start.is_nan() {
			if y < self.start[1] || y > self.end[1] {
				return None;
			}

			check_xy = false;
			check_yz = false;
		} else if y_start <= 0.0 && y_end <= 0.0 {
			return None;
		} else {
			if y_start > 0.0 {
				ray_min = y_start.min(ray_min);
			} else {
				ray_min = y_end.min(ray_min);
			}
		}

		if z_start.is_nan() {
			if z < self.start[2] || z > self.end[2] {
				return None;
			}

			check_xz = false;
			check_yz = false;
		} else if z_start <= 0.0 && z_end <= 0.0 {
			return None;
		} else {
			if z_start > 0.0 {
				ray_min = z_start.min(ray_min);
			} else {
				ray_min = z_end.min(ray_min);
			}
		}

		// Check y on x no overlap
		if check_xy && ((y_start < x_start && y_end < x_start) || (y_start > x_end && y_end > x_end)) {
			return None;
		}

		// Check z on x no overlap
		if check_xz && ((z_start < x_start && z_end < x_start) || (z_start > x_end && z_end > x_end)) {
			return None;
		}

		// Check z on y no overlap
		if check_yz && ((z_start < y_start && z_end < y_start) || (z_start > y_end && z_end > y_end)) {
			return None;
		}

		return Some(ray_min);
	}

	pub fn get_start(&self) -> &Vector3<f64> {
		&self.start
	}

	pub fn get_end(&self) -> &Vector3<f64> {
		&self.end
	}

	pub fn is_intersecting_plane(&self, position: Vector3<f64>, norm: Vector3<f64>) -> bool {
		let size = vec3_sub(self.end, self.start);
		let center = vec3_add(self.start, vec3_scale(size, 0.5));

		let p_len = vec3_dot([norm[0].abs(), norm[1].abs(), norm[2].abs()], size);
		let aabb_pos = vec3_sub(center, position);
		let dist = vec3_dot(norm, aabb_pos);

		return dist.abs() <= p_len;
	}

	pub fn is_intersecting_triangle(&self, position: Vector3<f64>, vec: [Vector3<f64>; 2], norm: Vector3<f64>) -> bool {
		// Compute points
		let p1 = position;
		let p2 = vec3_add(position, vec[0]);
		let p3 = vec3_add(position, vec[1]);

		// Compute edges
		let e1 = vec[0];
		let e2 = vec3_sub(vec[0], vec[1]);
		let e3 = vec3_neg(vec[1]);

		// AABB unit vectors
		let u1 = [1.0, 0.0, 0.0];
		let u2 = [0.0, 1.0, 0.0];
		let u3 = [0.0, 0.0, 1.0];

		let axis = [
			u1,
			u2,
			u3,
			norm,
			vec3_cross(u1, e1),
			vec3_cross(u2, e1),
			vec3_cross(u3, e1),
			vec3_cross(u1, e2),
			vec3_cross(u2, e2),
			vec3_cross(u3, e2),
			vec3_cross(u1, e3),
			vec3_cross(u2, e3),
			vec3_cross(u3, e3)
		];

		for a in axis.into_iter() {
			let (aabb_min, aabb_max) = get_aabb_interval_on_axis(*a, self.start, self.end);
			let (t_min, t_max) = get_point_interval_on_axis(*a, &[p1, p2, p3]);

			if t_min > aabb_max || aabb_min > t_max {
				return false;
			}
		}

		true
	}
}

impl Clone for AABB {
	fn clone(&self) -> Self {
		Self {
			start: self.start,
			end: self.end
		}
	}
}

fn project_points_onto_ray(ray: &RayTraceRay, points: (Vector3<f64>, Vector3<f64>)) -> (f64, f64, f64, f64, f64, f64) {
	let mut res = [0.0_f64; 6];
	let r_pos = ray.get_position();
	let r_dir = ray.get_direction();

	for dim in 0..3 {
		// Ray is orthogonal to this dimension
		if r_dir[dim].abs() < 1.0e-10 {
			res[dim << 1] = f64::NAN;
			res[(dim << 1) + 1] = f64::NAN;
			continue;
		}

		let start = (points.0[dim] - r_pos[dim]) / r_dir[dim];
		let end = (points.1[dim] - r_pos[dim]) / r_dir[dim];

		if end < start {
			res[dim << 1] = end;
			res[(dim << 1) + 1] = start;
		} else {
			res[dim << 1] = start;
			res[(dim << 1) + 1] = end;
		}
	}

	return (res[0], res[1], res[2], res[3], res[4], res[5]);
}

fn get_aabb_interval_on_axis(axis: Vector3<f64>, start: Vector3<f64>, end: Vector3<f64>) -> (f64, f64) {
	let points = [
		[start[0], start[1], start[2]],
		[start[0], start[1], end[2]],
		[start[0], end[1], start[2]],
		[start[0], end[1], end[2]],
		[end[0], start[1], start[2]],
		[end[0], start[1], end[2]],
		[end[0], end[1], start[2]],
		[end[0], end[1], end[2]]
	];

	get_point_interval_on_axis(axis, &points)
}

fn get_point_interval_on_axis(axis: Vector3<f64>, points: &[Vector3<f64>]) -> (f64, f64) {
	let mut o = (f64::NAN, f64::NAN);

	for p in points {
		let v = vec3_dot(axis, *p);
		o.0 = if o.0.is_nan() || o.0 >= v { v } else { o.0 };
		o.1 = if o.1.is_nan() || o.1 <= v { v } else { o.1 };
	}

	o

	/*points.iter()
		.map(|v| vec3_dot(axis, *v))
		.fold((f64::NAN, f64::NAN), |o, v| {
		(
			if o.0.is_nan() || o.0 > v { v } else { o.0 },
			if o.1.is_nan() || o.1 < v { v } else { o.1 }
		)})*/
}