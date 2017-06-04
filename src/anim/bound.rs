use vecmath::Vector3;

use anim::RayTraceAnim;
use anim::RayTraceBoundAnimation;
use anim::RayTraceSetPosition;
use anim::RayTraceSetRotation;

pub struct RayTraceAnimBoundRotation<'a> {
	animation: Box<RayTraceAnim<Vector3<f64>>>,
	object: Box<&'a mut (RayTraceSetRotation + Sync)>
}

impl<'a> RayTraceAnimBoundRotation<'a> {
	pub fn new(animation: Box<RayTraceAnim<Vector3<f64>>>, object: Box<&'a mut (RayTraceSetRotation + Sync)>) -> Self {
		Self {
			animation: animation,
			object: object
		}
	}
}

impl<'a> RayTraceBoundAnimation for RayTraceAnimBoundRotation<'a> {
	fn next_frame(&mut self, frame: usize) {
		self.object.set_rotation(self.animation.next_frame(frame));
	}
}

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
}