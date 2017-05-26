use std::f64;

use vecmath::Vector3;

use RayTraceRay;

#[allow(dead_code)]
pub struct AABB {
	start: Vector3<f64>,
	end: Vector3<f64>
}

#[allow(dead_code)]
impl AABB {
	pub fn new(x1: Vector3<f64>, x2: Vector3<f64>) -> Self {
		info!("AABB: {}, {}, {} to {}, {}, {}", x1[0], x1[1], x1[2], x2[0], x2[1], x2[2]);

		Self {
			start: [x1[0].min(x2[0]), x1[1].min(x2[1]), x1[2].min(x2[2])],
			end: [x1[0].max(x2[0]), x1[1].max(x2[1]), x1[2].max(x2[2])],
		}
	}

	pub fn is_hit(&self, ray: &RayTraceRay) -> bool {
		let [x, y, z] = *ray.get_position();
		let [dx, dy, dz] = *ray.get_direction();
		let (x_start, x_end, y_start, y_end, z_start, z_end) = project_points_onto_ray(ray, (self.start, self.end));
		let (mut check_xy, mut check_xz, mut check_yz) = (true, true, true);

		debug!("ray: {},{},{} -> {}, {}, {}", x, y, z, dx, dy, dz);
		debug!("bb: {}, {}, {} - {}, {}, {}", self.start[0], self.start[1], self.start[2], self.end[0], self.end[1], self.end[2]);
		debug!("x: {}-{}, y: {}-{}, z: {}-{}", x_start, x_end, y_start, y_end, z_start, z_end);

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