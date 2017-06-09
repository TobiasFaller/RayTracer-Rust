use std::mem;

use nonsync::UnsafeRef;

use object::RayTraceObject;

pub trait RayTraceAnim<T>: Send + Sync {
	fn first_frame(&self) -> T;
	fn get_frame(&self, frame: usize) -> T;
}

struct RayTraceBoundAnim<T, U> {
	animation: Box<RayTraceAnim<U>>,
	setter: Box<Fn(&mut T, U) + Send + Sync>,
	object_ref: UnsafeRef<T>
}

impl<T, U> RayTraceBoundAnim<T, U> {
	fn init(&self) {
		println!("Init");
		let ref animation = self.animation;
		println!("Animation");
		let first_frame = animation.first_frame();
		unsafe {
			let borrow = &mut *self.object_ref.get_mut();
			println!("Borrow");
			(self.setter)(borrow, first_frame);
			println!("Setter");
		}
	}

	fn step(&self, frame: usize) {
		println!("Step");
		unsafe {
			(self.setter)(&mut *self.object_ref.get_mut(), self.animation.get_frame(frame))
		}
	}
}

pub struct RayTraceAnimations {
	animations: Vec<RayTraceBoundAnim<Box<RayTraceObject>, f64>>
}

impl RayTraceAnimations {
	pub fn new() -> Self {
		Self {
			animations: Vec::new()
		}
	}

	pub fn init(&self) {
		for anim in self.animations.iter() {
			anim.init();
		}
	}

	pub fn frame(&self, frame: usize) {
		for anim in self.animations.iter() {
			anim.step(frame);
		}
	}

	pub fn add_animation<T: RayTraceObject, U>(&mut self, animation: Box<RayTraceAnim<U>>,
			setter: Box<Fn(&mut Box<T>, U) + Send + Sync>, object: UnsafeRef<Box<T>>) {
		unsafe {
			self.animations.push(mem::transmute(RayTraceBoundAnim {
				animation: animation,
				setter: setter,
				object_ref: object
			}));
		}
	}
}