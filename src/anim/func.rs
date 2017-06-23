use anim::RayTraceAnimation;

pub struct RayTraceAnimFunc<T> {
	func: Box<(Fn(usize) -> T) + Sync + Send>
}

impl<T> RayTraceAnimFunc<T> {
	pub fn new(func: Box<(Fn(usize) -> T) + Sync + Send>) -> Self {
		Self {
			func: func
		}
	}
}

impl<T> RayTraceAnimation<T> for RayTraceAnimFunc<T> {
	fn next_frame(&self, frame: usize) -> T {
		(self.func)(frame)
	}
}
