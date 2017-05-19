use vecmath::{Matrix3, Vector3, mat3_id, row_mat3_mul};

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

	if angle[0] != 0.0 {
		rot = rotate_x(angle[0]);
	}
	if angle[2] != 0.0 {
		rot = row_mat3_mul(rot, rotate_z(angle[2]));
	}
	if angle[1] != 0.0 {
		rot = row_mat3_mul(rot, rotate_y(angle[1]));
	}

	return rot;
}