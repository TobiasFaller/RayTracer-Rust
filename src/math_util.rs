use vecmath::{Matrix3, Vector3, mat3_id, row_mat3_mul};
use vecmath::{vec4_sub, vec4_scale};

use RayTraceRay;

#[allow(dead_code)]
pub fn rotate_z(angle: f64) -> Matrix3<f64> {
	let sin = angle.sin();
	let cos = angle.cos();

	[[cos, sin, 0.0], [-sin, cos, 0.0], [0.0, 0.0, 1.0]]
}

#[allow(dead_code)]
pub fn rotate_x(angle: f64) -> Matrix3<f64> {
	let sin = angle.sin();
	let cos = angle.cos();

	[[1.0, 0.0, 0.0], [0.0, cos, sin], [0.0, -sin, cos]]
}

#[allow(dead_code)]
pub fn rotate_y(angle: f64) -> Matrix3<f64> {
	let sin = angle.sin();
	let cos = angle.cos();

	[[cos, 0.0, -sin], [0.0, 1.0, 0.0], [sin, 0.0, cos]]
}

#[allow(dead_code)]
pub fn rotate_xyz(angle: Vector3<f64>) -> Matrix3<f64> {
	let mut rot = mat3_id();

	if angle[1] != 0.0 {
		rot = rotate_y(angle[1]);
	}
	if angle[0] != 0.0 {
		rot = row_mat3_mul(rot, rotate_x(angle[0]));
	}
	if angle[2] != 0.0 {
		rot = row_mat3_mul(rot, rotate_z(angle[2]));
	}

	return rot;
}

const THRESHOLD: f64 = 1e-10;

pub fn compute_plane_hit(ray: &RayTraceRay, center: Vector3<f64>, vec1: Vector3<f64>, vec2: Vector3<f64>)
		-> Option<(f64, f64, f64)> {
	let ray_pos = ray.get_position();
	let ray_dir = ray.get_direction();

	// Find the hitpoint using the Gauß-Jordan-algorithm
	let mut mat = [
			[ray_dir[0], -vec1[0], -vec2[0], center[0] - ray_pos[0]],
			[ray_dir[1], -vec1[1], -vec2[1], center[1] - ray_pos[1]],
			[ray_dir[2], -vec1[2], -vec2[2], center[2] - ray_pos[2]]
		];

	if mat[0][0].abs() < THRESHOLD {
		if mat[1][0].abs() < THRESHOLD {
			if mat[2][0].abs() < THRESHOLD {
				return None; // Cannot construct a hitpoint
			}

			// Swap row 0 and 2
			let tmp = mat[0];
			mat[0] = mat[2];
			mat[2] = tmp;
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
		}

		// Swap row 1 and 2
		let tmp = mat[1];
		mat[1] = mat[2];
		mat[2] = tmp;
	} else {
		if mat[2][1].abs() > THRESHOLD {
			mat[2] = vec4_sub(mat[2], vec4_scale(mat[1], mat[2][1] / mat[1][1]));
		}
	}

	if mat[2][2].abs() < THRESHOLD {
		return None; // Cannot construct a hitpoint
	}

	mat[0] = vec4_sub(mat[0], vec4_scale(mat[1], mat[0][1] / mat[1][1]));

	mat[0] = vec4_sub(mat[0], vec4_scale(mat[2], mat[0][2] / mat[2][2]));
	mat[1] = vec4_sub(mat[1], vec4_scale(mat[2], mat[1][2] / mat[2][2]));

	return Some((mat[0][3] / mat[0][0], mat[1][3] / mat[1][1], mat[2][3] / mat[2][2]));
}