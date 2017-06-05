use vecmath::Vector3;

use std::cell::UnsafeCell;

use anim::RayTraceAnim;
use anim::RayTraceBoundAnimation;
use anim::RayTraceSetRotation;

pub struct RayTraceAnimBoundRotation<'a, T: 'a + RayTraceSetRotation> {
	animation: Box<RayTraceAnim<Vector3<f64>> + Sync>,
	object: UnsafeCell<&'a mut Box<T>>
}

impl<'a, T: RayTraceSetRotation> RayTraceAnimBoundRotation<'a, T> {
	pub fn new(animation: Box<RayTraceAnim<Vector3<f64>> + Sync>, object: &'a mut Box<T>)
			-> Self {
		Self {
			animation: animation,
			object: UnsafeCell::from(object)
		}
	}
}

impl<'a, T: RayTraceSetRotation> RayTraceBoundAnimation for RayTraceAnimBoundRotation<'a, T> {
	fn next_frame(&self, frame: usize) where T: RayTraceSetRotation {
		unsafe { // Hack to modify object while other borrow is held
			let object: &'a mut Box<T> = *self.object.get();
			object.set_rotation(self.animation.next_frame(frame));
		}
	}
}

/*
pub struct RayTraceAnimBoundPosition<'a> {
	animation: Box<RayTraceAnim<Vector3<f64>>>,
	object: Box<&'a mut (RayTraceSetPosition + Sync)>
}

impl<'a> RayTraceAnimBoundPosition<'a> {
	pub fn new(animation: Box<RayTraceAnim<Vector3<f64>>>, object: Box<&'a mut (RayTraceSetPosition + Sync)>) -> Self {
		Self {
			animation: animation,
			object: object
		}
	}
}

impl<'a> RayTraceBoundAnimation for RayTraceAnimBoundPosition<'a> {
	fn next_frame(&mut self, frame: usize) {
		self.object.set_position(self.animation.next_frame(frame));
	}
}*/