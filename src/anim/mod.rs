use vecmath::Vector3;

pub trait RayTraceAnimVector3 {
	fn next_frame(&self, initial: Vector3<f64>, frame: usize) -> Vector3<f64>;
}