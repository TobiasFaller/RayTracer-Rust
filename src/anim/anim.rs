use vecmath::Vector3;

pub trait RayTraceAnim<T> {
	fn next_frame(&self, frame: usize) -> T;
}

pub trait RayTraceBoundAnimation {
	fn next_frame(&self, frame: usize);
}

pub trait RayTraceSetPosition {
	fn set_position(&mut self, position: Vector3<f64>);
}

pub trait RayTraceSetRotation {
	fn set_rotation(&mut self, rotation: Vector3<f64>);
}