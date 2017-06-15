use vecmath::Vector3;

const DEG_TO_RAD: f64 = 2.0 / 360.0 * 3.14159265359;

pub fn rot_deg(angle: Vector3<f64>) -> Vector3<f64> {
	[
		angle[0] * DEG_TO_RAD,
		angle[1] * DEG_TO_RAD,
		angle[2] * DEG_TO_RAD
	]
}