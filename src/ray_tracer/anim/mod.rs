use ray_tracer::vecmath::Vector3;

pub trait RayTraceAnimPosition {
	fn new_position(&self, position: Vector3<f64>, frame: usize) -> Vector3<f64>;
}

pub trait RayTraceAnimRotation {
	fn new_rotation(&self, position: Vector3<f64>, frame: usize) -> Vector3<f64>;
}