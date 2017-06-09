use vecmath::Vector3;

use anim::RayTraceAnim;

pub trait RayTraceBoundAnimation<'a>: Send + Sync {
	fn next_frame(&'a mut self, frame: usize);
}

pub struct RayTraceAnimBoundPosition<'a, 'b: 'a> {
	animation: Box<RayTraceAnim<'a, Vector3<f64>> + 'a>,
	setter: Box<FnMut(Vector3<f64>) + Send + Sync +'b>
}

impl<'a, 'b: 'a> RayTraceAnimBoundPosition<'a, 'b> {
	pub fn new(animation: Box<RayTraceAnim<'a, Vector3<f64>> + 'a>, setter: Box<FnMut(Vector3<f64>) + Send + Sync + 'b>)
			-> Self {
		Self {
			animation: animation,
			setter: setter
		}
	}
}

impl<'a, 'b: 'a> RayTraceBoundAnimation<'a> for RayTraceAnimBoundPosition<'a, 'b> {
	fn next_frame(&'a mut self, frame: usize) {
		(self.setter)(self.animation.next_frame(frame));
	}
}

/*pub struct RayTraceAnimBoundRotation {
	animation: Box<RayTraceAnim<Vector3<f64>>>,
	object: Box<RayTraceSetRotation + 'static>
}

impl RayTraceAnimBoundRotation {
	pub fn new(animation: Box<RayTraceAnim<Vector3<f64>>>, object: Box<RayTraceSetRotation + 'static>) -> Self {
		Self {
			animation: animation,
			object: object
		}
	}
}

impl RayTraceBoundAnimation for RayTraceAnimBoundRotation {
	fn next_frame(&mut self, frame: usize) {
		self.object.set_rotation(self.animation.next_frame(frame));
	}
}*/